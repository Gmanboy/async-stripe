use crate::config::{Client, Response};
use crate::ids::{CustomerId, PlanId, SubscriptionId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Customer, Discount, Invoice, PaymentMethod, PaymentSource, Plan, SubscriptionBillingThresholds,
    SubscriptionItem, TaxRate,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Subscription".
///
/// For more details see [https://stripe.com/docs/api/subscriptions/object](https://stripe.com/docs/api/subscriptions/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subscription {
    /// Unique identifier for the object.
    pub id: SubscriptionId,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    pub billing: SubscriptionBilling,

    /// Determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    pub billing_cycle_anchor: Timestamp,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// If the subscription has been canceled with the `at_period_end` flag set to `true`, `cancel_at_period_end` on the subscription will be true.
    ///
    /// You can use this attribute to determine whether a subscription that has a status of active is scheduled to be canceled at the end of the current period.
    pub cancel_at_period_end: bool,

    /// If the subscription has been canceled, the date of that cancellation.
    ///
    /// If the subscription was canceled with `cancel_at_period_end`, `canceled_at` will still reflect the date of the initial cancellation request, not the end of the subscription period when the subscription is automatically moved to a canceled state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// End of the current period that the subscription has been invoiced for.
    ///
    /// At the end of this period, a new invoice will be created.
    pub current_period_end: Timestamp,

    /// Start of the current period that the subscription has been invoiced for.
    pub current_period_start: Timestamp,

    /// ID of the customer who owns the subscription.
    pub customer: Expandable<Customer>,

    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// This value will be `null` for subscriptions where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If not set, defaults to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<PaymentSource>,

    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,

    /// Describes the current discount applied to this subscription, if there is one.
    ///
    /// When billing, a discount applied to a subscription overrides a discount applied on a customer-wide basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,

    /// If the subscription has ended, the date the subscription ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<Timestamp>,

    /// List of subscription items, each with an attached plan.
    pub items: List<SubscriptionItem>,

    /// The most recent invoice this subscription has generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_invoice: Option<Expandable<Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Hash describing the plan the customer is subscribed to.
    ///
    /// Only set if the subscription contains a single plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,

    /// The quantity of the plan to which the customer is subscribed.
    ///
    /// For example, if your plan is $10/user/month, and your customer has 5 users, you could pass 5 as the quantity to have the customer charged $50 (5 x $10) monthly.
    /// Only set if the subscription contains a single plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// Date of the last substantial change to this subscription.
    ///
    /// For example, a change to the items array, or a change of status, will reset this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Timestamp>,

    /// Date when the subscription was first created.
    ///
    /// The date might differ from the `created` date due to backdating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Timestamp>,

    /// Possible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`.
    ///
    ///   For `billing=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails.
    /// A subscription in this state can only have metadata and default_source updated.
    /// Once the first invoice is paid, the subscription moves into an `active` state.
    /// If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`.
    /// This is a terminal state, the open invoice will be voided and no further invoices will be generated.
    ///   A subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over.
    ///   If subscription `billing=charge_automatically` it becomes `past_due` when payment to renew it fails and `canceled` or `unpaid` (depending on your subscriptions settings) when Stripe has exhausted all payment retry attempts.
    ///   If subscription `billing=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that.
    /// Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed).
    /// After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices.
    pub status: SubscriptionStatus,

    /// If provided, each invoice created by this subscription will apply the tax rate, increasing the amount billed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    /// If the subscription has a trial, the end of that trial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Timestamp>,

    /// If the subscription has a trial, the beginning of that trial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_start: Option<Timestamp>,
}

impl Subscription {
    /// Creates a new subscription for a customer.
    ///
    /// For more details see https://stripe.com/docs/api#create_subscription.
    pub fn create(client: &Client, params: SubscriptionParams<'_>) -> Response<Subscription> {
        client.post_form("/subscriptions", params)
    }

    /// Retrieves the details of a subscription.
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_subscription.
    pub fn retrieve(client: &Client, subscription_id: &str) -> Response<Subscription> {
        client.get(&format!("/subscriptions/{}", subscription_id))
    }

    /// Updates a subscription's properties.
    /// For more details see https://stripe.com/docs/api#update_subscription.
    pub fn update(
        client: &Client,
        subscription_id: &str,
        params: SubscriptionParams<'_>,
    ) -> Response<Subscription> {
        client.post_form(&format!("/subscriptions/{}", subscription_id), params)
    }

    /// Cancels a subscription.
    ///
    /// For more details see https://stripe.com/docs/api#cancel_subscription.
    pub fn cancel(
        client: &Client,
        subscription_id: &str,
        params: CancelParams,
    ) -> Response<Subscription> {
        client.delete_query(&format!("/subscriptions/{}", subscription_id), params)
    }

    /// By default, returns a list of subscriptions that have not been canceled.
    ///
    /// In order to list canceled subscriptions, specify <code>status=canceled</code>.
    pub fn list(
        client: &Client,
        params: SubscriptionListParams<'_>,
    ) -> Response<List<Subscription>> {
        client.get_query("/subscriptions", &params)
    }
}

impl Object for Subscription {
    type Id = SubscriptionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CancelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_period_end: Option<bool>,
}

#[derive(Clone, Serialize, Debug)]
pub struct ItemParams<'a> {
    pub plan: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

/// The set of parameters that can be used when creating or updating a subscription.
///
/// For more details see https://stripe.com/docs/api#create_subscription and https://stripe.com/docs/api#update_subscription.
#[derive(Clone, Default, Serialize, Debug)]
pub struct SubscriptionParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ItemParams<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<TrialEnd<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u64>,
}

/// The parameters for `Subscription::list`.
#[derive(Clone, Debug, Serialize)]
pub struct SubscriptionListParams<'a> {
    /// The billing mode of the subscriptions to retrieve.
    ///
    /// Either `charge_automatically` or `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    billing: Option<SubscriptionBilling>,

    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    current_period_end: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    current_period_start: Option<RangeQuery<Timestamp>>,

    /// The ID of the customer whose subscriptions will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a SubscriptionId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// The ID of the plan whose subscriptions will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<PlanId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a SubscriptionId>,

    /// The status of the subscriptions to retrieve.
    ///
    /// One of: `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `unpaid`, `canceled`, or `all`.
    /// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
    /// Passing in a value of `all` will return subscriptions of all statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<SubscriptionStatusFilter>,
}

impl<'a> SubscriptionListParams<'a> {
    pub fn new() -> Self {
        SubscriptionListParams {
            billing: Default::default(),
            created: Default::default(),
            current_period_end: Default::default(),
            current_period_start: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            plan: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `Subscription`'s `billing` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionBilling {
    ChargeAutomatically,
    SendInvoice,
}

/// An enum representing the possible values of an `Subscription`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

/// An enum representing the possible values of an `SubscriptionListParams`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatusFilter {
    Active,
    All,
    Canceled,
    Ended,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum TrialEnd<'a> {
    Timestamp(Timestamp),
    Special(&'a str),
}
