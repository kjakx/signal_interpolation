use dasp_sample::*;
use dasp_signal::{self as signal, Signal, interpolate::Converter};
use dasp_interpolate::linear::Linear;
use hound;

fn main() {
    let mut signal = signal::rate(44100.0).const_hz(440.0).sine();
    let mut interp = Linear::new(0.0_f32, 0.0_f32);
    let mut converted = Converter::from_hz_to_hz(signal, interp, 440.0, 523.25).into_source();
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine_C.wav", spec).unwrap();
    (0..44100).for_each(|n| {
        writer.write_sample(converted.next().to_sample::<i16>()).unwrap();
    });
}
