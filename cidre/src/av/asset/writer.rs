use std::intrinsics::transmute;

use crate::{arc, av, cm, define_cls, define_obj_type, ns, objc, ut};

use super::WriterInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    /// Indicates that the status of the asset writer is not currently known.
    Unknown = 0,
    /// Indicates that the asset writer is successfully writing samples to its output file.
    Writing = 1,
    /// Indicates that the asset writer has successfully written all samples following a call to finishWriting.
    Completed = 2,
    /// Indicates that the asset writer can no longer write samples to its output file because of an error. The error is described by the value of the asset writer's error property.
    Failed = 3,
    /// Indicates that the asset writer can no longer write samples because writing was canceled with the cancelWriting method.
    Cancelled = 4,
}

define_obj_type!(pub Writer(ns::Id));

impl arc::A<Writer> {
    #[objc::msg_send(initWithURL:fileType:error:)]
    pub fn init_with_url_file_type_err<'ar>(
        self,
        url: &ns::Url,
        file_type: &av::FileType,
        error: &mut Option<&'ar ns::Error>,
    ) -> Option<arc::R<Writer>>;

    #[objc::msg_send(initWithContentType:)]
    pub unsafe fn init_with_content_type_throws(
        self,
        output_content_type: &ut::Type,
    ) -> arc::R<Writer>;
}

impl Writer {
    define_cls!(AV_ASSET_WRITER);

    #[objc::msg_send(shouldOptimizeForNetworkUse)]
    pub fn should_optimize_for_network_use(&self) -> bool;

    #[objc::msg_send(setShouldOptimizeForNetworkUse:)]
    pub fn set_should_optimize_for_network_use(&mut self, value: bool);

    #[objc::msg_send(canAddInput:)]
    pub fn can_add_input(&self, input: &WriterInput) -> bool;

    #[objc::msg_send(addInput:)]
    pub unsafe fn add_input_throws(&mut self, input: &WriterInput);

    pub fn add_input<'ar>(&mut self, input: &WriterInput) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.add_input_throws(input) })
    }

    /// Prepares the receiver for accepting input and for writing its output to its output file.
    #[objc::msg_send(startWriting)]
    pub fn start_writing(&mut self) -> bool;

    #[objc::msg_send(startSessionAtSourceTime:)]
    pub fn start_session_at_source_time(&mut self, start_time: cm::Time);

    #[objc::msg_send(endSessionAtSourceTime:)]
    pub fn end_session_at_source_time(&mut self, end_time: cm::Time);

    #[objc::msg_send(finishWriting)]
    pub fn finish_writing(&mut self);

    #[objc::msg_send(cancelWriting)]
    pub fn cancel_writing(&mut self);

    #[objc::msg_send(status)]
    pub fn status(&self) -> Status;

    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<&ns::Error>;

    #[objc::msg_send(inputs)]
    pub fn inputs(&self) -> &ns::Array<WriterInput>;

    /// ```
    /// use cidre::{av, ns};
    /// let url = ns::Url::with_str("file://tmp/bla.mp4").unwrap();
    ///
    /// let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
    /// assert_eq!(writer.inputs().len(), 0);
    /// ```
    pub fn with_url_and_file_type<'ar>(
        url: &ns::Url,
        file_type: &av::FileType,
    ) -> Result<arc::R<Self>, &'ar ns::Error> {
        let mut error = None;
        unsafe {
            let res = Self::alloc().init_with_url_file_type_err(url, file_type, &mut error);
            match error {
                None => Ok(transmute(res)),
                Some(e) => Err(e),
            }
        }
    }

    pub fn with_content_type<'ar>(
        output_content_type: &ut::Type,
    ) -> Result<arc::R<Self>, &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            Self::alloc().init_with_content_type_throws(output_content_type)
        })
    }
}

/// AVAssetWriterSegmentation
impl Writer {
    #[objc::msg_send(preferredOutputSegmentInterval)]
    pub fn preferred_output_segment_interval(&self) -> cm::Time;

    /// Specifies preferred segment interval.
    #[objc::msg_send(setPreferredOutputSegmentInterval:)]
    pub fn set_preferred_output_segment_interval(&mut self, val: cm::Time);

    #[objc::msg_send(initialSegmentStartTime)]
    pub fn initial_segment_start_time(&self) -> cm::Time;

    /// Specifies start time of initial segment.
    ///
    /// A numeric time must be set if the value of preferredOutputSegmentInterval property
    /// is positive numeric. If not, this property is irrelevant.
    ///
    /// This property cannot be set after writing has started.
    /// TODO: check throws
    #[objc::msg_send(setInitialSegmentStartTime:)]
    pub fn set_initial_segment_start_time(&mut self, val: cm::Time);

    #[objc::msg_send(outputFileTypeProfile)]
    pub fn output_file_type_profile(&self) -> Option<&av::FileTypeProfile>;

    /// TODO: check throws
    #[objc::msg_send(setOutputFileTypeProfile:)]
    pub fn set_output_file_type_profile(&mut self, val: Option<&av::FileTypeProfile>);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<&ns::Id>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

    /// This method throws an exception if the delegate method to output segment data is not implemented,
    /// or if the value of the preferredOutputSegmentInterval property is not kCMTimeIndefinite.
    #[objc::msg_send(flushSegment)]
    pub unsafe fn flush_segment_throws(&mut self);

    /// Closes the current segment and outputs it to the -assetWriter:didOutputSegmentData:segmentType:segmentReport:
    /// or -assetWriter:didOutputSegmentData:segmentType: delegate method.
    pub fn flush_segment<'ar>(&mut self) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.flush_segment_throws() })
    }
}

#[objc::obj_trait]
pub trait Delegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(assetWriter:didOutputSegmentData:segmentType:segmentReport:)]
    fn asset_writer_did_output_segment_data_with_report(
        &mut self,
        writer: &av::AssetWriter,
        segment_data: &ns::Data,
        segment_type: av::AssetSegmentType,
        segment_report: Option<&av::AssetSegmentReport>,
    );

    #[objc::optional]
    #[objc::msg_send(assetWriter:didOutputSegmentData:segmentType:)]
    fn asset_writer_did_output_segment_data(
        &mut self,
        writer: &av::AssetWriter,
        segment_data: &ns::Data,
        segment_type: av::AssetSegmentType,
    );
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_ASSET_WRITER: &'static objc::Class<Writer>;
}

#[cfg(test)]
mod tests {
    use crate::ut;

    #[test]
    fn basics() {
        use crate::{av, ns};
        let url = ns::Url::with_str("file://tmp/bla.mp4").unwrap();

        let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
        assert_eq!(writer.inputs().len(), 0);

        av::AssetWriter::with_content_type(&ut::Type::pdf())
            .expect_err("Can't create writer for pdf");
        av::AssetWriter::with_content_type(&ut::Type::mpeg4movie()).expect("Can't create viedeo");
    }
}
