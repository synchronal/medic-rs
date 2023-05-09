#[derive(Debug, Eq, PartialEq)]
pub enum DepStatus {
    DepOk,
    Outdated,
}

#[derive(Debug)]
pub struct Dep {
    pub name: String,
    pub status: DepStatus,
}

impl Dep {
    pub fn from_deps_output(output: String) -> Result<Vec<Dep>, Box<dyn std::error::Error>> {
        let deps: Vec<Dep> = output
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(Dep::parse_dep)
            .collect();

        Ok(deps)
    }

    fn parse_dep(output: &[&str]) -> Dep {
        let (info_line, _lock, status_line) = (output[0], output[1], output[2]);
        let info: Vec<&str> = info_line.split(' ').collect();

        let status = match status_line.trim() {
            "ok" => DepStatus::DepOk,
            _ => DepStatus::Outdated,
        };

        Dep {
            name: info[1].to_owned(),
            status,
        }
    }
}
