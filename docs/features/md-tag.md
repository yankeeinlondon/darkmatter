# `md` tag within Markdown

When a markdown parser encounters HTML block elements it let's the _block element_ style the content and no longer treats the interior scope as markdown. So for example:

```md
This is markdown content.

<table><tr>
<td>
    left _hand_ content
</td>
<td>
    right _hand_ content
</td>
</tr></table>
```

There's no problem putting HTML into markdown document _but_ the content within the `<td>...</td>` blocks will NOT be treated as markdown, just as plain text. Often this is what you want but in cases where you do want that interior content to be treated as Markdown you can simply add the `md` tag to your HTML like so:

```md
This is markdown content.

<table><tr>
<td md>
    left _hand_ content
</td>
<td md>
    right _hand_ content
</td>
</tr></table>
```

Regardless of how nested into the HTML the block element is, if the `md` tag is present it that block will first be parsed as Markdown before evaluating the containing page.
