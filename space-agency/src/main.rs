use personnel::AstronautJob;
use personnel::Candidate;

fn get_job_code(job: AstronautJob) -> i32 {
    match job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

fn calculate_job_score(_candidate: &Candidate) {
    let mut job_score = get_job_code((&_candidate).primary_job)
        * match _candidate.secondary_job == None {
            true => get_job_code((&_candidate).primary_job),
            false => get_job_code((&_candidate).secondary_job.unwrap()),
        };

    if job_score > 576 {
        job_score = job_score % 576;
    }
}

fn main() {
    let _candidates = Candidate::load_candidate_file();

    calculate_job_score(&_candidates[1]);
}
