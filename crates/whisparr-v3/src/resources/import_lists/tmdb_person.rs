use core_macros::fields_blob;

/// TMDb Person import list — imports titles associated with a specific person.
///
/// At least one of the credit-type toggles must be enabled or the API rejects
/// the list on save.
#[fields_blob(
    implementation = "TMDbPersonImport",
    config_contract = "TMDbPersonSettings"
)]
pub struct TmdbPersonConfig {
    /// TMDb person identifier whose associated titles are imported.
    #[wire(name = "personId")]
    pub person_id: Option<String>,
    /// Include titles where the person appears as a cast member.
    #[wire(name = "personCast")]
    pub person_cast: Option<bool>,
    /// Include titles where the person is credited as director.
    #[wire(name = "personCastDirector")]
    pub person_cast_director: Option<bool>,
    /// Include titles where the person is credited as producer.
    #[wire(name = "personCastProducer")]
    pub person_cast_producer: Option<bool>,
    /// Include titles where the person has a sound department credit.
    #[wire(name = "personCastSound")]
    pub person_cast_sound: Option<bool>,
    /// Include titles where the person is credited as a writer.
    #[wire(name = "personCastWriting")]
    pub person_cast_writing: Option<bool>,
}
