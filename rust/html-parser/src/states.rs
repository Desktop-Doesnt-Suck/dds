
// pub enum InsertionMode {
//     Initial,
//     BeforeHtml,
//     BeforeHead,
//     InHead,
//     InHeadNoScript,
//     AfterHead,
//     InBody,
//     Text,
//     InTable,
//     InTableText,
//     InCaption,
//     InColumnGroup,
//     InTableBody,
//     InRow,
//     InCell,
//     InSelect,
//     InSelectTable,
//     AfterBody,
//     InFrameset,
//     AfterFrameset,
//     AfterAfterBody,
//     AfterAfterFrameset,
// }

pub enum TokenizationState  {
    Data,
    CharacterReferenceInData,
    // RCDATA,
    // CharacterReferenceInRCDATA,
    // RAWTEXT,
    // ScriptData,
    // PLAINTEXT,
    TagOpen,
    EndTagOpen,
    TagName,
    // RCDATALessThanSign,
    // RCDATAEndTagOpen,
    // RCDATAEndTagName,
    // RAWTEXTLessThanSign,
    // RAWTEXTEndTagOpen,
    // RAWTEXTEndTagName,
    // ScriptDataLessThanSign,
    // ScriptDataEndTagOpen,
    // ScriptDataEndTagName,
    // ScriptDataEscapeStart,
    // ScriptDataEscapeStartDash,
    // ScriptDataEscaped,
    // ScriptDataEscapedDash,
    // ScriptDataEscapedDashDash,
    // ScriptDataEscapedLessThanSign,
    // ScriptDataEscapedEndTagOpen,
    // ScriptDataEscapedEndTagName,
    // ScriptDataDoubleEscapeStart,
    // ScriptDataDoubleEscaped,
    // ScriptDataDoubleEscapedDash,
    // ScriptDataDoubleEscapedDashDash,
    // ScriptDataDoubleEscapedLessThanSign,
    // ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    // AfterAttributeName,
    // BeforeAttributeValue,
    // AttributeValueDoubleQuoted,
    // AttributeValueSingleQuoted,
    // AttributeValueUnquoted,
    // CharacterrReferenceInAttributeValue,
    // AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    // CommentStart,
    // CommentStartDash,
    // Comment,
    // CommentEndDash,
    // CommentEnd,
    // CommentEndBang,
    // DOCTYPE,
    // BeforeDOCTYPEName,
    // DOCTYPEName,
    // AfterDOCTYPEName,
    // AfterDOCTYPEPublicKeyword,
    // BeforeDOCTYPEPublicIdentifier,
    // DOCTYPEPublicIdentifierDoubleQuoted,
    // DOCTYPEPublicIdentifierSingleQuoted,
    // AfterDOCTYPEPublicIdentifier,
    // BetweenDOCTYPEPublicAndSystemIdentifiers,
    // AfterDOCTYPESystemKeyword,
    // BeforeDOCTYPESystemIdentifier,
    // DOCTYPESystemIdentifierDoubleQuoted,
    // DOCTYPESystemIdentifierSingleQuoted,
    // AfterDOCTYPESystemIdentifier,
    // BogusDOCTYPE,
    // CDATASection,
}