use std::mem::transmute;

use crate::{cf, define_obj_type, ns, vn};

define_obj_type!(DetectBarcodesRequest(vn::ImageBasedRequest));

impl DetectBarcodesRequest {
    pub const REVISION_1: usize = 1;
    pub const REVISION_2: usize = 2;
    pub const REVISION_3: usize = 3;

    pub fn results(&self) -> Option<&cf::ArrayOf<vn::BarcodeObservation>> {
        unsafe { transmute(rsel_results(self)) }
    }

    pub fn new() -> cf::Retained<Self> {
        unsafe { VNDetectBarcodesRequest_new() }
    }

    pub fn symbologies(&self) -> &cf::ArrayOf<vn::BarcodeSymbology> {
        unsafe { rsel_symbologies(self) }
    }

    pub fn set_symbologies(&mut self, value: &cf::ArrayOf<vn::BarcodeSymbology>) {
        unsafe { wsel_setSymbologies(self, value) }
    }

    pub fn supported_symbologies<'ar>(
        &self,
    ) -> Result<&cf::ArrayOf<vn::BarcodeSymbology>, &'ar cf::Error> {
        unsafe {
            let mut error = None;
            let res = rsel_supportedSymbologiesAndReturnError(self, &mut error);
            if let Some(r) = res {
                Ok(r)
            } else {
                Err(error.unwrap())
            }
        }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_results(id: &ns::Id) -> Option<&cf::Array>;

    fn VNDetectBarcodesRequest_new() -> cf::Retained<DetectBarcodesRequest>;

    fn rsel_symbologies(id: &ns::Id) -> &cf::ArrayOf<vn::BarcodeSymbology>;

    fn wsel_setSymbologies(id: &mut ns::Id, value: &cf::ArrayOf<vn::BarcodeSymbology>);

    fn rsel_supportedSymbologiesAndReturnError<'ar, 'a>(
        id: &'a ns::Id,
        error: &mut Option<&'ar cf::Error>,
    ) -> Option<&'a cf::ArrayOf<vn::BarcodeSymbology>>;
}

#[cfg(test)]
mod tests {
    use crate::vn;

    #[test]
    fn basics() {
        let request = vn::DetectBarcodesRequest::new();
        let symbologies = request.symbologies();

        assert!(!symbologies.is_empty());

        let supported_symbologies = request.supported_symbologies().unwrap();

        assert!(supported_symbologies.contains(vn::BarcodeSymbology::qr()));
        assert!(symbologies.contains(vn::BarcodeSymbology::qr()));
    }
}