use std::collections::HashMap;

type PrintJob = Vec<u32>;
pub struct Printer {
    pages_followers: HashMap<u32, Vec<u32>>,
    jobs: Vec<PrintJob>,
}

impl Printer {
    pub fn new(input: &str) -> Self {
        let mut pages_followers: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut jobs = Vec::new();

        let mut reading_jobs = false;
        for line in input.lines() {
            if line.is_empty() {
                reading_jobs = true;
                continue;
            }

            if !reading_jobs {
                let parts: Vec<&str> = line.split("|").collect();
                let page_id = parts[0].parse().unwrap();
                let follower = parts[1].parse().unwrap();

                if let Some(followers) = pages_followers.get_mut(&page_id) {
                    followers.push(follower);
                } else {
                    pages_followers.insert(page_id, vec![follower]);
                }
            } else {
                let job = line
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect();
                jobs.push(job);
            }
        }

        Self { pages_followers, jobs }
    }

    pub fn get_jobs(&self) -> &Vec<PrintJob> {
        &self.jobs
    }

    pub fn get_page_followers(&self, page_id: u32) -> Option<&Vec<u32>> {
        self.pages_followers.get(&page_id)
    }

    pub fn is_valid_job(&self, job: &PrintJob) -> bool {
        let mut printed = Vec::new();
        for page_id in job {
            if let Some(followers) = self.get_page_followers(*page_id) {
                if followers.iter().any(|follower| printed.contains(follower)) {
                    return false;
                }
            }
            printed.push(*page_id);
        }
        true
    }

    pub fn get_valid_jobs(&self) -> Vec<&PrintJob> {
        self.jobs
            .iter()
            .filter(|job| self.is_valid_job(job))
            .collect()
    }

    pub fn get_invalid_jobs(&self) -> Vec<&PrintJob> {
        self.jobs
            .iter()
            .filter(|job| !self.is_valid_job(job))
            .collect()
    }

    pub fn fix_job(&self, job: &PrintJob) -> PrintJob {
        let mut job_to_fix = job.clone();
        // sort the page_ids using their followers
        // every page_id should be printed before its followers
        job_to_fix.sort_by(|a, b| {
            let def_a = vec![];
            let followers_a = self.get_page_followers(*a).unwrap_or(&def_a);

            let def_b = vec![];
            let followers_b = self.get_page_followers(*b).unwrap_or(&def_b);

            if followers_a.contains(b) {
                std::cmp::Ordering::Greater
            } else if followers_b.contains(a) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        job_to_fix
    }
}
