# Audio normalization

Audio normalization for both video and audio files based on EBU R 128.

## Requirements

ffmpeg

## Supported file formats

- mp3
- mp4
- mov

## How do I want to use it?

It should be a command with the following basic features

- help
- color output
- multiple input option
- progress bar
- debug options / levels

### Strategies to support (next level)

There are several common audio normalization standards used in the industry. Here are a few examples:

- Peak Normalization: Peak normalization adjusts the audio so that the highest peak in the waveform reaches a specified level, often 0 dB. This method simply amplifies or attenuates the audio uniformly, without considering the overall loudness perception.

- RMS Normalization: Root Mean Square (RMS) normalization measures the average power of the audio signal over time and adjusts the gain to achieve a desired RMS level. RMS normalization takes into account the overall loudness of the audio, making it more suitable for achieving a consistent perceived loudness.

- LUFS/LKFS Normalization: Loudness Units Full Scale (LUFS), also known as Loudness K-weighted Full Scale (LKFS), is a widely adopted standard for loudness normalization. It measures the perceived loudness of audio content using specific weighting filters to approximate human hearing. LUFS normalization ensures consistent loudness levels across different audio tracks and platforms, such as broadcasting and streaming services. EBU R 128 is a recommendation based on LUFS normalization.

- ITU-R BS.1770: ITU-R BS.1770 is a standard developed by the International Telecommunication Union (ITU) that provides guidelines for measuring and normalizing audio program loudness. It specifies algorithms for measuring short-term loudness (momentary), integrated loudness (average over a defined duration), and loudness range (LRA) to achieve consistent loudness levels.

- AES Streaming Loudness Recommendation (AES-41): AES-41 is an audio loudness recommendation developed by the Audio Engineering Society (AES) for streaming and online delivery. It focuses on delivering audio content with consistent loudness levels across various platforms and devices.

It's important to note that different platforms, industries, and countries may have specific requirements or variations in audio normalization standards. It's recommended to check the guidelines and specifications of the target platform or medium to ensure compliance with their specific standards.

## Contibution

Do whatever you want
