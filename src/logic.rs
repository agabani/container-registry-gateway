pub enum AdmitError {
    NotMonitored,
    CriticalVulnerability,
    HighVulnerability,
    MediumVulnerability,
    LowVulnerability,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum ProjectCriticality {
    Critical = 4,
    High = 3,
    Medium = 2,
    Low = 1,
    None = 0,
}

/// Checks if the project is admitted.
pub(crate) fn admitted(
    response: &crate::snyk::organization_projects_post::Response,
) -> Result<(), AdmitError> {
    if let Some(project) = response.body.projects.first() {
        let criticality = &project.attributes.criticality;
        let project_criticality = if criticality.contains(&"critical".to_string()) {
            ProjectCriticality::Critical
        } else if criticality.contains(&"high".to_string()) {
            ProjectCriticality::High
        } else if criticality.contains(&"medium".to_string()) {
            ProjectCriticality::Medium
        } else if criticality.contains(&"low".to_string()) {
            ProjectCriticality::Low
        } else {
            ProjectCriticality::None
        };

        let issue_count = &project.issue_counts_by_severity;
        if issue_count.critical > 0 && project_criticality < ProjectCriticality::Critical {
            Err(AdmitError::CriticalVulnerability)
        } else if issue_count.high > 0 && project_criticality < ProjectCriticality::High {
            Err(AdmitError::HighVulnerability)
        } else if issue_count.medium > 0 && project_criticality < ProjectCriticality::Medium {
            Err(AdmitError::MediumVulnerability)
        } else if issue_count.low > 0 && project_criticality < ProjectCriticality::Low {
            Err(AdmitError::LowVulnerability)
        } else {
            Ok(())
        }
    } else {
        Err(AdmitError::NotMonitored)
    }
}

impl std::fmt::Display for AdmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdmitError::NotMonitored => {
                write!(f, "Image not monitored for vulnerabilities")
            }
            AdmitError::CriticalVulnerability => {
                write!(f, "Image exceeded vulnerability threshold critical")
            }
            AdmitError::HighVulnerability => {
                write!(f, "Image exceeded vulnerability threshold high")
            }
            AdmitError::MediumVulnerability => {
                write!(f, "Image exceeded vulnerability threshold medium")
            }
            AdmitError::LowVulnerability => {
                write!(f, "Image exceeded vulnerability threshold low")
            }
        }
    }
}
