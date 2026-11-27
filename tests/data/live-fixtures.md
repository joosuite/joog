# Live fixtures

The live suite copies these files to a temporary directory before upload. They
contain synthetic content and no personal data. The checked-in media files are
ready to upload; the tests do not require media-generation tools.

| File | Content | Purpose |
| --- | --- | --- |
| `live-fixture.mp4` | Four seconds of a generated 320 × 180 test pattern with silent AAC audio | Private video uploads, captions, comments, and slide video insertion |
| `live-fixture.png` | A generated 256 × 256 green image | Contact photos and square playlist images |
| `live-thumbnail.png` | A generated 1280 × 720 green image | YouTube custom thumbnails |
| `live-banner.png` | A generated 2048 × 1152 green image | YouTube banner upload with the original banner saved and restored |
| `live-document-fixture.docx` | Three tracked insertions by separate synthetic reviewers and one anchored image | Suggestion acceptance/rejection and positioned-image deletion after conversion to Google Docs |

The DOCX is an ordinary OOXML ZIP package containing `word/document.xml`,
`word/settings.xml`, relationship files, and the synthetic PNG. Its tracked
insertions use `w:ins`; its floating image uses `wp:anchor` with square wrapping.
Google's [conversion guidance](https://support.google.com/docs/answer/6033474?hl=en)
describes how Office tracked changes become Docs suggestions. Imported authors
can still prevent the document owner from deleting a suggestion, which the live
suite records separately from successful acceptance and rejection.
