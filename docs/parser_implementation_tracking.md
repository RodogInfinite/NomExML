The following tables need to be verified as of June 2, 2023

---
## Extensible Markup Language (XML) 1.0 (Fifth Edition)
### W3C Recommendation 26 November 2008
---
| Production Rule | Name | Definition | Implemented | Tested | :bug: |
|:----------------------:|:----------|-------------|:-----------:|:------:|:----------:|
|   [1]  | document        | prolog element Misc* | :heavy_check_mark: | :x: |
|   [2]  | Char            | #x9 \| #xA \| #xD \| [#x20-#xD7FF] \| [#xE000-#xFFFD] \| [#x10000-#x10FFFF] | :heavy_check_mark: | :x: |
|   [3]  | S               | (#x20 \| #x9 \| #xD \| #xA)+ | :heavy_check_mark: | :x: |
|   [4]  | NameStartChar   | ":" \| [A-Z] \| "_" \| [a-z] \| [#xC0-#xD6] \| [#xD8-#xF6] \| [#xF8-#x2FF] \| [#x370-#x37D] \| [#x37F-#x1FFF] \| [#x200C-#x200D] \| [#x2070-#x218F] \| [#x2C00-#x2FEF] \| [#x3001-#xD7FF] \| [#xF900-#xFDCF] \| [#xFDF0-#xFFFD] \| [#x10000-#xEFFFF] | :heavy_check_mark: | :x: |
|   [4a] | NameChar        | NameStartChar \| "-" \| "." \| [0-9] \| #xB7 \| [#x0300-#x036F] \| [#x203F-#x2040] | :heavy_check_mark: | :x: |
|   [5]  | Name            | NameStartChar (NameChar)* | :heavy_check_mark: | :x: |
|   [6]  | Names           | Name (#x20 Name)* | :heavy_check_mark: | :x: |
|   [7]  | Nmtoken         | (NameChar)+ | :heavy_check_mark: | :x: |
|   [8]  | Nmtokens        | Nmtoken (#x20 Nmtoken)* | :heavy_check_mark: | :x: |
|   [9]  | EntityValue     | '"' ([^%&"] \| PEReference \| Reference)* '"' \| "'" ([^%&'] \| PEReference \| Reference)* "'" | :heavy_check_mark: | :x: |
|   [10] | AttValue        | '"' ([^<&"] \| Reference)* '"' \| "'" ([^<&'] \| Reference)* "'" | :heavy_check_mark: | :x: |
|   [11] | SystemLiteral   | ('"' [^"]* '"') \| ("'" [^']* "'") | :heavy_check_mark: | :x: |
|   [12] | PubidLiteral    | '"' PubidChar* '"' \| "'" (PubidChar - "'")* "'" | :heavy_check_mark: | :x: |
|   [13] | PubidChar       | #x20 \| #xD \| #xA \| [a-zA-Z0-9] \| [-'()+,./:=?;!*#@$_%] | :heavy_check_mark: | :x: |
|   [14] | CharData        | [^<&]* - ([^<&]* ']]>' [^<&]*) | :heavy_check_mark: | :x: |
|   [15] | Comment         | '<!--' ((Char - '-') \| ('-' (Char - '-')))* '-->' | :heavy_check_mark: | :x: |
|   [16] | PI              | '<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>' | :heavy_check_mark: | :x: |
|   [17] | PITarget        | Name - (('X' \| 'x') ('M' \| 'm') ('L' \| 'l')) | :heavy_check_mark: | :x: |
|   [18] | CDSect          | CDStart CData CDEnd | :heavy_check_mark: | :x: | :question: |
|   [19] | CDStart         | '<![CDATA[' | :heavy_check_mark: | :x: | :question: |
|   [20] | CData           | (Char* - (Char* ']]>' Char*)) | :heavy_check_mark: | :x: | :question: |
|   [21] | CDEnd           | ']]>' | :heavy_check_mark: | :x: | :question: |
|   [22] | prolog          | XMLDecl? Misc* (doctypedecl Misc*)? | :heavy_check_mark: | :x: |
|   [23] | XMLDecl         | '<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>' | :heavy_check_mark: | :x: |
|   [24] | VersionInfo     | S 'version' Eq ("'" VersionNum "'" \| '"' VersionNum '"') | :heavy_check_mark: | :x: |
|   [25] | Eq              | S? '=' S? | :heavy_check_mark: | :x: |
|   [26] | VersionNum      | '1.' [0-9]+ | :heavy_check_mark: | :x: |
|   [27] | Misc            | Comment \| PI \| S | :heavy_check_mark: | :x: |
|   [28]  | doctypedecl     | '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>' | :heavy_check_mark: | :x: |
|   [28a] | DeclSep         | PEReference \| S | :heavy_check_mark: | :x: |
|   [28b] | intSubset       | (markupdecl \| DeclSep)* | :heavy_check_mark: | :x: |
|   [29]  | markupdecl      | elementdecl \| AttlistDecl \| EntityDecl \| NotationDecl \| PI \| Comment | :heavy_check_mark: | :x: |
|   [30]  | extSubset       | TextDecl? extSubsetDecl | :heavy_check_mark: | :x: |
|   [31]  | extSubsetDecl   | (markupdecl \| conditionalSect \| DeclSep)* | :heavy_check_mark: | :x: |
|   [32]  | SDDecl          | S 'standalone' Eq (("'" ('yes' \| 'no') "'") \| ('"' ('yes' \| 'no') '"')) | :heavy_check_mark: | :x: |
|   [39]  | element         | EmptyElemTag \| STag content ETag | :heavy_check_mark: | :x: |
|   [40]  | STag            | '<' Name (S Attribute)* S? '>' | :heavy_check_mark: | :x: |
|   [41]  | Attribute       | Name Eq AttValue | :heavy_check_mark: | :x: |
|   [42]  | ETag            | '</' Name S? '>' | :heavy_check_mark: | :x: |
|   [43]  | content         | CharData? ((element \| Reference \| CDSect \| PI \| Comment) CharData?)* | :heavy_check_mark: | :x: |
|   [44]  | EmptyElemTag    | '<' Name (S Attribute)* S? '/>' | :heavy_check_mark: | :x: |
|   [45]  | elementdecl      | '<!ELEMENT' S Name S contentspec S? '>' | :heavy_check_mark: | :x: |
|   [46]  | contentspec      | 'EMPTY' \| 'ANY' \| Mixed \| children | :heavy_check_mark: | :x: |
|   [47]  | children         | (choice \| seq) ('?' \| '*' \| '+')? | :heavy_check_mark: | :x: |
|   [48]  | cp               | (Name \| choice \| seq) ('?' \| '*' \| '+')? | :heavy_check_mark: | :x: |
|   [49]  | choice           | '(' S? cp ( S? '\|' S? cp )+ S? ')' | :heavy_check_mark: | :x: |
|   [50]  | seq              | '(' S? cp ( S? ',' S? cp )* S? ')' | :heavy_check_mark: | :x: |
|   [51]  | Mixed            | '(' S? '#PCDATA' (S? '\|' S? Name)* S? ')*' \| '(' S? '#PCDATA' S? ')' | :heavy_check_mark: | :x: |
|   [52]  | AttlistDecl      | '<!ATTLIST' S Name AttDef* S? '>' | :heavy_check_mark: | :x: |
|   [53]  | AttDef           | S Name S AttType S DefaultDecl | :heavy_check_mark: | :x: |
|   [54]  | AttType          | StringType \| TokenizedType \| EnumeratedType | :heavy_check_mark: | :x: |
|   [55]  | StringType       | 'CDATA' | :heavy_check_mark: | :x: |
|   [56]  | TokenizedType    | 'ID'\| 'IDREF'\| 'IDREFS'\| 'ENTITY' \| 'ENTITIES' \| 'NMTOKEN' \| 'NMTOKENS' | :heavy_check_mark: | :x: |
|   [57]  | EnumeratedType   | NotationType \| Enumeration | :heavy_check_mark: | :x: |
|   [58]  | NotationType     | 'NOTATION' S '(' S? Name (S? '\|' S? Name)* S? ')' | :heavy_check_mark: | :x: |
|   [59]  | Enumeration      | '(' S? Nmtoken (S? '\|' S? Nmtoken)* S? ')' | :heavy_check_mark: | :x: ||   [60]  | DefaultDecl      | '#REQUIRED' \| '#IMPLIED' \| (('#FIXED' S)? AttValue) | :heavy_check_mark: | :x: |
|   [61]  | conditionalSect  | includeSect \| ignoreSect | :x: | :x: |
|   [62]  | includeSect      | '<![' S? 'INCLUDE' S? '[' extSubsetDecl ']]>' | :x: | :x: |
|   [63]  | ignoreSect       | '<![' S? 'IGNORE' S? '[' ignoreSectContents* ']]>' | :x: | :x: |
|   [64]  | ignoreSectContents | Ignore ('<![' ignoreSectContents ']]>' Ignore)* | :x: | :x: |
|   [65]  | Ignore           | Char* - (Char* ('<![' \| ']]>') Char*) | :x: | :x: |
|   [66]  | CharRef          | '&#' [0-9]+ ';' \| '&#x' [0-9a-fA-F]+ ';' | :heavy_check_mark: | :x: |
|   [67]  | Reference        | EntityRef \| CharRef | :heavy_check_mark: | :x: |
|   [68]  | EntityRef        | '&' Name ';' | :heavy_check_mark: | :x: |
|   [69]  | PEReference      | '%' Name ';' | :heavy_check_mark: | :x: |
|   [70]  | EntityDecl       | GEDecl \| PEDecl | :heavy_check_mark: | :x: |
|   [71]  | GEDecl           | '<!ENTITY' S Name S EntityDef S? '>' | :heavy_check_mark: | :x: |
|   [72]  | PEDecl           | '<!ENTITY' S '%' S Name S PEDef S? '>' | :heavy_check_mark: | :x: | :question: |
|   [73]  | EntityDef        | EntityValue \| (ExternalID NDataDecl?) | :heavy_check_mark: | :x: |
|   [74]  | PEDef            | EntityValue \| ExternalID | :heavy_check_mark: | :x: |
|   [75]  | ExternalID       | 'SYSTEM' S SystemLiteral \| 'PUBLIC' S PubidLiteral S SystemLiteral | :heavy_check_mark: | :x: |
|   [76]  | NDataDecl        | S 'NDATA' S Name | :heavy_check_mark: | :x: |
|   [77]  | TextDecl         | '<?xml' VersionInfo? EncodingDecl S? '?>' | :x: | :x: |
|   [78]  | extParsedEnt     | TextDecl? content | :x: | :x: |
|   [80]  | EncodingDecl     | S 'encoding' Eq ('"' EncName '"' \| "'" EncName "'" ) | :heavy_check_mark: | :x: |
|   [81]  | EncName          | [A-Za-z] ([A-Za-z0-9._] \| '-')* | :heavy_check_mark: | :x: |
|   [82]  | NotationDecl     | '<!NOTATION' S Name S (ExternalID \| PublicID) S? '>' | :heavy_check_mark: | :x: |
|   [83]  | PublicID         | 'PUBLIC' S PubidLiteral | :heavy_check_mark: | :x: |
|   [84]  | Letter           | BaseChar \| Ideographic | :x: | :x: |
|   [85]  | BaseChar   |  ...  | :x: | :x: |
|   [86]  | Ideographic      | [#x4E00-#x9FA5] \| #x3007 \| [#x3021-#x3029] | :x: | :x: |
|   [87]  | CombiningChar   |  ...  | :x: | :x: |
|   [88]  | Digit            | [#x0030-#x0039] \| [#x0660-#x0669] \| [#x06F0-#x06F9] \| [#x0966-#x096F] \| [#x09E6-#x09EF] \| [#x0A66-#x0A6F] \| [#x0AE6-#x0AEF] \| [#x0B66-#x0B6F] \| [#x0BE7-#x0BEF] \| [#x0C66-#x0C6F] \| [#x0CE6-#x0CEF] \| [#x0D66-#x0D6F] \| [#x0E50-#x0E59] \| [#x0ED0-#x0ED9] \| [#x0F20-#x0F29] | :x: | :x: |
|   [89]  | Extender         | #x00B7 \| #x02D0 \| #x02D1 \| #x0387 \| #x0640 \| #x0E46 \| #x0EC6 \| #x3005 \| [#x3031-#x3035] \| [#x309D-#x309E] \| [#x30FC-#x30FE] | :x: | :x: |


---
## Namespaces in XML 1.0 (Third Edition)
### W3C Recommendation 8 December 2009
---
| Production Rule | Name | Definition | Implemented | Tested | :bug: |
|:----------------------:|:----------|-------------|:-----------:|:------:|:----------:|
| [1] | NSAttName | PrefixedAttName \| DefaultAttName | :heavy_check_mark: | :x: |
| [2] | PrefixedAttName | 'xmlns:' NCName | :heavy_check_mark: | :x: |
| [3] | DefaultAttName | 'xmlns' | :heavy_check_mark: | :x: |
| [4] | NCName | Name - (Char* ':' Char*) | :heavy_check_mark: | :x: |
| [5] | NCNameChar | NameChar - ':' | :heavy_check_mark: | :x: |
| [6] | NCNameStartChar | NCName - ( Char Char Char* ) | :heavy_check_mark: | :x: |
| [7] | QName | PrefixedName \| UnprefixedName | :heavy_check_mark: | :x: |
| [8] | PrefixedName | Prefix ':' LocalPart | :heavy_check_mark: | :x: |
| [9] | UnprefixedName | LocalPart | :heavy_check_mark: | :x: |
| [10] | Prefix | NCName | :heavy_check_mark: | :x: |
| [11] | LocalPart | NCName | :heavy_check_mark: | :x: |
| [12] | STag | '<' QName (S Attribute)* S? '>' | :heavy_check_mark: | :x: |
| [13] | ETag | '</' QName S? '>' | :heavy_check_mark: | :x: |
| [14] | EmptyElemTag | '<' QName (S Attribute)* S? '/>' | :heavy_check_mark: | :x: |
| [15] | Attribute | NSAttName Eq AttValue \| QName Eq AttValue | :heavy_check_mark: | :x: |
| [16] | doctypedecl | '<!DOCTYPE' S QName (S ExternalID)? S? ('[' (markupdecl \| PEReference \| S)* ']' S?)? '>' | :heavy_check_mark: | :x: |
| [17] | elementdecl | '<!ELEMENT' S QName S contentspec S? '>' | :heavy_check_mark: | :x: |
| [18] | cp | (QName \| choice \| seq) ('?' \| '*' \| '+')? | :heavy_check_mark: | :x: |
| [19] | Mixed | '(' S? '#PCDATA' (S? '\|' S? QName)* S? ')*' \| '(' S? '#PCDATA' S? ')' | :heavy_check_mark: | :x: |
| [20] | AttlistDecl | '<!ATTLIST' S QName AttDef* S? '>' | :heavy_check_mark: | :x: |
| [21] | AttDef | S (QName \| NSAttName) S AttType S DefaultDecl | :heavy_check_mark: | :x: |
