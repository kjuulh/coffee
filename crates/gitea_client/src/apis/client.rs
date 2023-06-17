use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  activitypub_api: Box<::apis::ActivitypubApi>,
  admin_api: Box<::apis::AdminApi>,
  issue_api: Box<::apis::IssueApi>,
  miscellaneous_api: Box<::apis::MiscellaneousApi>,
  notification_api: Box<::apis::NotificationApi>,
  organization_api: Box<::apis::OrganizationApi>,
  package_api: Box<::apis::PackageApi>,
  repository_api: Box<::apis::RepositoryApi>,
  settings_api: Box<::apis::SettingsApi>,
  user_api: Box<::apis::UserApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      activitypub_api: Box::new(::apis::ActivitypubApiClient::new(rc.clone())),
      admin_api: Box::new(::apis::AdminApiClient::new(rc.clone())),
      issue_api: Box::new(::apis::IssueApiClient::new(rc.clone())),
      miscellaneous_api: Box::new(::apis::MiscellaneousApiClient::new(rc.clone())),
      notification_api: Box::new(::apis::NotificationApiClient::new(rc.clone())),
      organization_api: Box::new(::apis::OrganizationApiClient::new(rc.clone())),
      package_api: Box::new(::apis::PackageApiClient::new(rc.clone())),
      repository_api: Box::new(::apis::RepositoryApiClient::new(rc.clone())),
      settings_api: Box::new(::apis::SettingsApiClient::new(rc.clone())),
      user_api: Box::new(::apis::UserApiClient::new(rc.clone())),
    }
  }

  pub fn activitypub_api(&self) -> &::apis::ActivitypubApi{
    self.activitypub_api.as_ref()
  }

  pub fn admin_api(&self) -> &::apis::AdminApi{
    self.admin_api.as_ref()
  }

  pub fn issue_api(&self) -> &::apis::IssueApi{
    self.issue_api.as_ref()
  }

  pub fn miscellaneous_api(&self) -> &::apis::MiscellaneousApi{
    self.miscellaneous_api.as_ref()
  }

  pub fn notification_api(&self) -> &::apis::NotificationApi{
    self.notification_api.as_ref()
  }

  pub fn organization_api(&self) -> &::apis::OrganizationApi{
    self.organization_api.as_ref()
  }

  pub fn package_api(&self) -> &::apis::PackageApi{
    self.package_api.as_ref()
  }

  pub fn repository_api(&self) -> &::apis::RepositoryApi{
    self.repository_api.as_ref()
  }

  pub fn settings_api(&self) -> &::apis::SettingsApi{
    self.settings_api.as_ref()
  }

  pub fn user_api(&self) -> &::apis::UserApi{
    self.user_api.as_ref()
  }


}
