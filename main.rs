use ffmpeg_next as ffmpeg;

fn main() {
    // Initialize the FFmpeg library
    ffmpeg::init().unwrap();

    // Open the input file
    let input_file = "path/to/input/file.mp4";
    let input_ctx = ffmpeg::format::input(&input_file).unwrap();

    // Open the output file
    let output_file = "path/to/output/file.mp4";
    let mut output_ctx = ffmpeg::format::output(&output_file).unwrap();

    // Iterate over the input streams
    for (stream_index, stream) in input_ctx.streams().enumerate() {
        // Find the best decoder for the stream
        let decoder = stream.codec().decoder().audio().unwrap_or_else(|| {
            stream.codec().decoder().video().unwrap_or_else(|| {
                panic!("Unsupported stream type for stream index {}", stream_index);
            })
        });

        // Open the decoder
        decoder.set_parameters(stream.parameters()).unwrap();

        // Add a stream to the output file
        let mut output_stream = output_ctx
            .add_stream(decoder.codec().id())
            .unwrap();

        // Set the stream parameters
        output_stream.set_parameters(stream.parameters()).unwrap();

        // Create the frame and packet
        let mut frame = ffmpeg::frame::Frame::new();
        let mut packet = ffmpeg::packet::Packet::empty();

        // Initialize the normalization filter
        let filter_graph = ffmpeg::filter::graph::Graph::new();
        let normalizer = filter_graph.add(
            &ffmpeg::filter::find("loudnorm").unwrap(),
            "loudnorm",
        );
        normalizer
            .get()
            .set("I", "-23")
            .set("LRA", "7")
            .set("TP", "-2")
            .set("print_format", "summary")
            .set("linear", "true")
            .set("measured_I", "false")
            .set("measured_LRA", "false")
            .set("measured_TP", "false")
            .set("measured_thresh", "false")
            .set("offset", "true");

        // Configure the filter graph
        let mut inputs = filter_graph.inputs();
        let mut outputs = filter_graph.outputs();
        inputs.add(stream.index(), 0);
        outputs.add(output_stream.index(), 0);
        filter_graph.configure().unwrap();

        // Iterate over the input frames
        while let Ok(_) = input_ctx.read_frame(&mut packet) {
            // Skip frames that don't belong to the current stream
            if packet.stream_index() != stream.index() {
                continue;
            }

            // Decode the packet into a frame
            decoder.send_packet(&packet).unwrap();
            while decoder.receive_frame(&mut frame).is_ok() {
                // Filter the frame
                filter_graph.run(&mut frame).unwrap();

                // Encode and write the frame to the output file
                output_stream.codec().encoder().unwrap().send_frame(&frame).unwrap();
                while output_stream.codec().encoder().unwrap().receive_packet(&mut packet).is_ok() {
                    output_ctx.write_frame(&packet).unwrap();
                }
            }
        }

        // Flush the encoder
        decoder.send_eof().unwrap();
        while decoder.receive_frame(&mut frame).is_ok() {
            // Filter the frame
            filter_graph.run(&mut frame).unwrap();

            // Encode and write the frame to the output file
            output_stream.codec().encoder().unwrap().send_frame(&frame).unwrap();
            while output_stream.codec().encoder().unwrap().receive_packet(&mut packet).is_ok() {
                output_ctx.write_frame(&packet).unwrap();
            }
        }
    }

    // Write the output file trailer
    output_ctx.write_trailer().unwrap();
}