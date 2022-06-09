

#[cfg(test)]

use personnel::Candidate;
use personnel::AstronautJob;
mod tests {
    #[test]
    fn it_works() {
        let a = Candidate {
            primary_job: AstronautJob::Biogeochemist, 
            secondary_job: AstronautJob::Biogeochemist, 
            age: 25, 
            health: 72};
        assert_eq!(calculate_job_score(&a), 217);
    }
}
