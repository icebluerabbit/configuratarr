use core_macros::fields_blob;

/// TMDb Company import list — imports movies produced by a specific company.
#[fields_blob(
    implementation = "TMDbCompanyImport",
    config_contract = "TMDbCompanySettings"
)]
pub struct TmdbCompanyConfig {
    /// TMDb company identifier whose productions are imported.
    #[wire(name = "companyId")]
    pub company_id: Option<String>,
}
