use cf = core_foundation;
use cf::array::{CFArray, CFArrayRef};
use cf::base::{
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFIndex,
    CFRange,
    CFTypeID,
    CFTypeRef,
    CFWrapper,
    kCFAllocatorDefault,
};
use cf::dictionary::{CFDictionary, CFDictionaryRef, UntypedCFDictionary};
use cf::number::CFNumber;
use cf::set::CFSet;
use cf::string::{CFString, CFStringRef};

use font_descriptor::{
    CTFontAttributes,
    CTFontDescriptor,
    CTFontDescriptorCreateMatchingFontDescriptors,
    CTFontDescriptorRef,
};
use font_manager::CTFontManagerCopyAvailableFontFamilyNames;

use libc::c_void;

struct __CTFontCollection { private: () }
pub type CTFontCollectionRef = *__CTFontCollection;

impl CTFontCollectionRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

pub type CTFontCollection = CFWrapper<CTFontCollectionRef, (), ()>;

pub trait CTFontCollectionMethods {
    static fn new_from_descriptors(descs: &CFArray<CTFontDescriptorRef>) -> CTFontCollection;
    static fn create_for_all_families() -> CTFontCollection;
    static fn create_for_family(family: &str) -> CTFontCollection;
    static pure fn get_family_names() -> CFArray<CFStringRef>;
    pure fn get_descriptors() -> CFArray<CTFontDescriptorRef>;
}

pub impl CTFontCollection : CTFontCollectionMethods {
    static fn new_from_descriptors(descs: &CFArray<CTFontDescriptorRef>) -> CTFontCollection {
        let key = CFString::wrap_extern(kCTFontCollectionRemoveDuplicatesOption);
        let value = CFNumber::new(1_i8);
        let options = CFDictionary::new([ (*key.borrow_ref(), *value.borrow_type_ref()) ]);
        let result = CTFontCollectionCreateWithFontDescriptors(*descs.borrow_ref(), *options.borrow_ref());
        CFWrapper::wrap_owned(result)
    }

    static fn create_for_all_families() -> CTFontCollection {
        let key = CFString::wrap_extern(kCTFontCollectionRemoveDuplicatesOption);
        let value = CFNumber::new(1_i8);
        let options = CFDictionary::new([ (*key.borrow_ref(), *value.borrow_type_ref()) ]);
        let result = CTFontCollectionCreateFromAvailableFonts(*options.borrow_ref());
        CFWrapper::wrap_owned(result)
    }

    static fn create_for_family(family: &str) -> CTFontCollection unsafe {
        use font_descriptor::kCTFontFamilyNameAttribute;
 
        let family_attr = CFString::wrap_extern(kCTFontFamilyNameAttribute);
        let family_name = CFString::new(family);

        let specified_attrs : CTFontAttributes = CFDictionary::new([
            ( *family_attr.borrow_ref(), *family_name.borrow_type_ref() )
        ]);

        // XXX FIXME: this doesn't work
        //let wildcard_desc : CTFontDescriptor = font_descriptor::new_from_attributes(&specified_attrs);
        fail;
/*
        let mandatory_attrs = CFSet::new([ *family_attr.borrow_ref() ]);
        let matched_descs = CTFontDescriptorCreateMatchingFontDescriptors(*wildcard_desc.borrow_ref(),
                                                                          *mandatory_attrs.borrow_ref());

        let matched_descs : CFArray<CTFontDescriptorRef> = CFWrapper::wrap_owned(matched_descs);
*/
        // I suppose one doesn't even need the CTFontCollection object at this point.
        // But we stick descriptors into and out of it just to provide a nice wrapper API.
/*
        font_collection::new_from_descriptors(&matched_descs)
*/
    }

    static pure fn get_family_names() -> CFArray<CFStringRef> unsafe {
        CFWrapper::wrap_owned(CTFontManagerCopyAvailableFontFamilyNames())
    }

    pure fn get_descriptors() -> CFArray<CTFontDescriptorRef> unsafe {
        use cf::base::CFRetain;

        // surprise! this function follows the Get rule, despite being named *Create*.
        // So we have to addRef it to avoid CTFontCollection from double freeing it later.
        let wrapper : CFArray<CTFontDescriptorRef> = CFWrapper::wrap_shared(CTFontCollectionCreateMatchingFontDescriptors(self.obj));
        return move wrapper;
    }
}

extern {
    /*
     * CTFontCollection.h
     */

    const kCTFontCollectionRemoveDuplicatesOption: CFStringRef;

    fn CTFontCollectionCreateCopyWithFontDescriptors(original: CTFontCollectionRef,
                                                     descriptors: CFArrayRef,
                                                     options: CFDictionaryRef) -> CTFontCollectionRef;
    fn CTFontCollectionCreateFromAvailableFonts(options: CFDictionaryRef) -> CTFontCollectionRef;
    // this stupid function doesn't actually do any wildcard expansion; 
    // it just chooses the best match. Use
    // CTFontDescriptorCreateMatchingDescriptors instead.
    fn CTFontCollectionCreateMatchingFontDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
    fn CTFontCollectionCreateWithFontDescriptors(descriptors: CFArrayRef,
                                                 options: CFDictionaryRef) -> CTFontCollectionRef;
    //fn CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback;
    fn CTFontCollectionGetTypeID() -> CFTypeID;
}