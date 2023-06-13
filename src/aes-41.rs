use ffmpeg_next as ffmpeg;
use ffmpeg_next::format::sample_rate::Samples;

fn main() {
    // Initialize the FFmpeg library
    ffmpeg::init().unwrap();

    // Input file path
    let input_file = "path/to/input/file.mp4";

    // Open the input file
    let input_ctx = ffmpeg::format::input(&input_file).unwrap();

    // Find the audio stream
    let audio_stream = input_ctx
        .streams()
        .best(ffmpeg::media::Type::Audio)
        .unwrap();

    // Get audio stream information
    let audio_stream_index = audio_stream.index();
    let audio_codec_params = audio_stream.parameters();
    let audio_duration = audio_stream.duration().unwrap_or_default();
    let audio_sample_rate = audio_codec_params.sample_rate();
    let audio_channels = audio_codec_params.channel_layout().channels();

    // Open the audio decoder
    let audio_decoder = audio_stream.codec().decoder().audio().unwrap();
    audio_decoder.set_parameters(audio_codec_params).unwrap();

    // Create the audio frame and packet
    let mut audio_frame = ffmpeg::frame::audio::Audio::new(
        ffmpeg::media::audio::SampleFormat::FLTP,
        audio_sample_rate,
        audio_channels,
    );
    let mut audio_packet = ffmpeg::packet::Packet::empty();

    // Create the loudness measurement filter
    let filter_graph = ffmpeg::filter::graph::Graph::new();
    let loudness_meter = filter_graph.add(
        &ffmpeg::filter::find("loudnorm").unwrap(),
        "loudnorm",
    );
    loudness_meter
        .get()
        .set("linear", "true")
        .set("print_format", "summary");

    // Configure the filter graph
    let mut inputs = filter_graph.inputs();
    let mut outputs = filter_graph.outputs();
    inputs.add(audio_stream_index, 0);
    outputs.add(loudness_meter.index(), 0);
    filter_graph.configure().unwrap();

    // Calculate the target loudness using AES-41 recommendation (-16 LUFS)
    let target_loudness = -16.0;

    // Calculate the loudness normalization gain
    let loudness_filter_output = filter_graph.run(&mut audio_frame).unwrap();
    let loudness_measurement = loudness_filter_output.get(loudness_meter.index(), 0).unwrap();
    let measured_loudness = loudness_measurement
        .metadata()
        .iter()
        .find(|meta| meta.key() == "input_i").unwrap()
        .value()
        .parse::<f64>()
        .unwrap();
    let gain = target_loudness - measured_loudness;

    // Create the audio resampler
    let mut audio_resampler = ffmpeg::software::resampling::context::Context::get(
        audio_decoder.format(),
        audio_decoder.channels(),
        ffmpeg::media::audio::SampleFormat::FLTP,
        audio_decoder.sample_rate(),
        audio_decoder.channels(),
        ffmpeg::media::audio::SampleFormat::FLTP,
        audio_decoder.sample_rate(),
    ).unwrap();

    // Output file path
    let output_file = "path/to/output/file.mp4";

    // Open the output file
    let mut output_ctx = ffmpeg::format::output(&output_file).unwrap();

    // Add an audio stream to the output file
    let mut output_stream = output_ctx
        .add_stream::<ffmpeg::codec::encoder::Audio>(ffmpeg::codec::id::AUDIO_AAC)
        .unwrap();

    // Set the audio stream parameters
    output_stream.set_parameters(audio_codec_params).unwrap();

    // Open the audio encoder
    let audio_encoder = output_stream.codec().encoder().audio().unwrap();
    audio_encoder.set_parameters(output_stream.parameters()).unwrap();

    // Initialize packet counters
    let mut processed_samples = 0;

    // Iterate over the input audio frames
    while let Ok(_) = input_ctx.read_frame(&mut audio_packet) {
        // Skip frames that don't belong to the audio stream
        if audio_packet.stream_index() != audio_stream_index {
            continue;
        }

        // Decode the packet into a frame
        audio_decoder.send_packet(&audio_packet).unwrap();
        while audio_decoder.receive_frame(&mut audio_frame).is_ok() {
            // Resample the audio frame
            audio_resampler
                .run(
                    &audio_frame.data(),
                    audio_frame.nb_samples(),
                    &mut audio_frame.data_mut(),
                    audio_frame.nb_samples(),
                )
                .unwrap();

            // Apply loudness normalization gain to audio samples
            let audio_samples = audio_frame.data_mut()[0].as_mut_slice::<f32>();
            for sample in audio_samples.iter_mut() {
                *sample *= 10.0f32.powf(gain / 20.0);
            }

            // Encode and write the frame to the output file
            output_stream
                .codec()
                .encoder()
                .unwrap()
                .send_frame(&audio_frame)
                .unwrap();
            while output_stream.codec().encoder().unwrap().receive_packet(&mut audio_packet).is_ok() {
                output_ctx.write_frame(&audio_packet).unwrap();
            }

            // Update processed samples counter
            processed_samples += audio_frame.nb_samples();
        }

        // Stop processing if the audio duration is reached
        if audio_duration > 0 && processed_samples >= audio_duration {
            break;
        }
    }

    // Write the output file trailer
    output_ctx.write_trailer().unwrap();
}