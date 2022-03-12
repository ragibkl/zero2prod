use super::subscriber_email::SubscriberEmail;
use super::subscriber_name::SubscriberName;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
