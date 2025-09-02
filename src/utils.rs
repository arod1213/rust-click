pub fn tempo_to_samples(tempo: u64, sample_rate: u32) -> u64 {
    if tempo == 0 {
        panic!("tempo can not be 0");
    }

    let ms_per_beat = 60 * 1000 / tempo;
    ms_per_beat * sample_rate as u64 / 1000
}

mod test {
    #[cfg(test)]
    use super::*;
    #[test]
    fn test_tempo_to_samples() {
        let tempo: u64 = 120;
        let sr: u32 = 44100;
        let samples_per_beat = tempo_to_samples(tempo, sr);
        assert_eq!(22050, samples_per_beat);

        let tempo: u64 = 240;
        let samples_per_beat = tempo_to_samples(tempo, sr);
        assert_eq!(11025, samples_per_beat);

        let tempo: u64 = 60;
        let samples_per_beat = tempo_to_samples(tempo, sr);
        assert_eq!(44100, samples_per_beat);

        let tempo: u64 = 30;
        let samples_per_beat = tempo_to_samples(tempo, sr);
        assert_eq!(88200, samples_per_beat);

        let tempo: u64 = 300;
        let samples_per_beat = tempo_to_samples(tempo, sr);
        assert_eq!(8820, samples_per_beat);
    }
}
