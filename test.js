import { expect } from '@esm-bundle/chai';

describe('common-markdown', () => {
    it('should import module and parse markdown to HTML', async () => {
        const { parse } = await import('./pkg/common-markdown.js');

        const markdown = "# Common Markdown\n\nThis is a paragraph";
        const expectedHtml = "<h1>Common Markdown</h1>\n<p>This is a paragraph</p>\n";

        const html = parse(markdown);

        expect(html).to.equal(expectedHtml);
    });
});