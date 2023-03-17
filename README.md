# common-markdown

A WebAssembly wrapper for the fast [pulldown-cmark](https://crates.io/crates/pulldown-cmark) parser for the standard version of Markdown, [CommonMark](https://commonmark.org/).

It is built with [wasm-pack](https://rustwasm.github.io/wasm-pack/), to provide a wasm version that can be ran in JavaScript browser environments to parse Markdown into HTML in a fast and easy way.

## Usage

**NOTE:** When parsing HTML for use in a browser, it is neccecary to also filter the content
from any malicious code injection. A good package to use for this is
[DOMPurify](https://github.com/cure53/DOMPurify).

```javascript
import DOMPurify from 'https://unpkg.com/dompurify@3.0.1/dist/purify.es.js';

// Dynamic import is needed in order for the module to complete its memory initialization before usage
const { parse } = await import('https://unpkg.com/common-markdown@0.1.0/common-markdown.js');

const markdown = `
# CommonMark

A strongly defined, highly compatible specification of Markdown

## What is Markdown?

It’s a plain text format for writing structured documents, based on formatting conventions from email and usenet.

[Learn Markdown in 60 Seconds](https://commonmark.org/help/)

## Who created Markdown?

It was [developed in 2004 by John Gruber in collaboration with Aaron Swartz](https://en.wikipedia.org/wiki/Markdown#History). Gruber wrote the first markdown-to-html converter in Perl, and it soon became widely used in websites. By 2014 there were dozens of implementations in many languages.

## Why is CommonMark needed?

John Gruber’s [canonical description of Markdown’s syntax](https://daringfireball.net/projects/markdown/syntax) does not specify the syntax unambiguously.

In the absence of a spec, early implementers consulted the original \`Markdown.pl\` code to resolve these ambiguities. But \`Markdown.pl\` was quite buggy, and gave manifestly bad results in many cases, so it was not a satisfactory replacement for a spec. Markdown.pl was last updated December 17th, 2004.

...
`;

const html = DOMPurify.sanitize(parse(markdown));

document.body.innerHTML = html;
```