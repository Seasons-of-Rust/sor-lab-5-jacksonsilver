#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a = Candidate {
            primary_job: AstronautJob::Biogeochemist, 
            secondary_job: None, 
            age: 25, 
            health: 72};
        assert_eq!(calculate_job_score(&a), 217);
    }
}
