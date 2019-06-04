// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{ChargeId, RefundId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{BalanceTransaction, Charge, Currency, TransferReversal};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Refund".
///
/// For more details see [https://stripe.com/docs/api/refunds/object](https://stripe.com/docs/api/refunds/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Refund {
    /// Unique identifier for the object.
    pub id: RefundId,

    /// Amount, in %s.
    pub amount: i64,

    /// Balance transaction that describes the impact on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// ID of the charge that was refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Expandable<Charge>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// (Available on non-card refunds only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// If the refund failed, this balance transaction describes the adjustment made on your account balance that reverses the initial balance transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// If the refund failed, the reason for refund failure if known.
    ///
    /// Possible values are `lost_or_stolen_card`, `expired_or_canceled_card`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Reason for the refund.
    ///
    /// If set, possible values are `duplicate`, `fraudulent`, and `requested_by_customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// This is the transaction number that appears on email receipts sent for this refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,

    /// The transfer reversal that is associated with the refund.
    ///
    /// Only present if the charge came from another Stripe account.
    /// See the Connect documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transfer_reversal: Option<Expandable<TransferReversal>>,

    /// Status of the refund.
    ///
    /// For credit card refunds, this can be `pending`, `succeeded`, or `failed`.
    /// For other types of refunds, it can be `pending`, `succeeded`, `failed`, or `canceled`.
    /// Refer to our [refunds](https://stripe.com/docs/refunds#failed-refunds) documentation for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// If the accompanying transfer was reversed, the transfer reversal object.
    ///
    /// Only applicable if the charge was created using the destination parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_reversal: Option<Expandable<TransferReversal>>,
}

impl Refund {
    /// Returns a list of all refunds you’ve previously created.
    ///
    /// The refunds are returned in sorted order, with the most recent refunds appearing first.
    /// For convenience, the 10 most recent refunds are always available by default on the charge object.
    pub fn list(client: &Client, params: ListRefunds<'_>) -> Response<List<Refund>> {
        client.get_query("/refunds", &params)
    }

    /// Create a refund.
    pub fn create(client: &Client, params: CreateRefund<'_>) -> Response<Refund> {
        client.post_form("/refunds", &params)
    }

    /// Retrieves the details of an existing refund.
    pub fn retrieve(client: &Client, id: &RefundId, expand: &[&str]) -> Response<Refund> {
        client.get_query(&format!("/refunds/{}", id), &Expand { expand })
    }

    /// Updates the specified refund by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  This request only accepts `metadata` as an argument.
    pub fn update(client: &Client, id: &RefundId, params: UpdateRefund<'_>) -> Response<Refund> {
        client.post_form(&format!("/refunds/{}", id), &params)
    }
}

impl Object for Refund {
    type Id = RefundId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "refund"
    }
}

/// The parameters for `Refund::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateRefund<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<ChargeId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>,
}

impl<'a> CreateRefund<'a> {
    pub fn new() -> Self {
        CreateRefund {
            amount: Default::default(),
            charge: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            reason: Default::default(),
            refund_application_fee: Default::default(),
            reverse_transfer: Default::default(),
        }
    }
}

/// The parameters for `Refund::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListRefunds<'a> {
    /// Only return refunds for the charge specified by this charge ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<ChargeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a RefundId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<RefundId>,
}

impl<'a> ListRefunds<'a> {
    pub fn new() -> Self {
        ListRefunds {
            charge: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `Refund::update`.
#[derive(Clone, Debug, Serialize)]
pub struct UpdateRefund<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<'a> UpdateRefund<'a> {
    pub fn new() -> Self {
        UpdateRefund { expand: Default::default(), metadata: Default::default() }
    }
}

/// An enum representing the possible values of an `CreateRefund`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

impl RefundReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            RefundReason::Duplicate => "duplicate",
            RefundReason::Fraudulent => "fraudulent",
            RefundReason::RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl AsRef<str> for RefundReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RefundReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
