# Icons in Darkmatter

Adding icons to your markdown content in Darkmatter is easy with the help of the Iconify ecosystem and powerful [`unplugin-icons`](https://github.com/antfu/unplugin-icons) integrated into the ViteJS pipeline.

Simply add in an icon such as `<mdi:checkbox-marked-circle />` in your markdown and it will be converted into an SVG icon at build time. For a full list of icons take a look at the interactive catalogue at: [Iconify Icons](https://icones.js.org/collection/all).

> Note: if you install the **Darkmatter** language server and configure it to be used for `.md` files (it defaults only to `.dm` files) then it will convert the icons visually into editors which support it.

## Global Configuration

For those of you who are used to using the `unplugin-icons` library you'll know typically that you'd need to install this library and then introduce it into your `vite.config.ts` file along with the configuration settings it provides.

This is entirely available to you as an option if you would like to do this or you have existing project that already has. However, since Darkmatter is "aware" of this plugin you can simply just use the syntax in your markdown and when Darkmatter detects the use of the Iconify syntax in your Markdown it will be added to the ViteJS pipeline with a effective configuration for your project.

## Icon Level Configuration

Icons can be styled by use of either the `style` or `class` tags. With `style` you're getting to add in useful CSS properties like:

```md
<mdi:account-edit style="font-size: 2em; color: blue />"
```

but if you prefer you can also use `class` and target either your own CSS classes or take advantage of Tailwind CSS classes:

```md
<mdi:account-edit class="font-xl text-blue-500" />
```

::aside

For those interested, the ability to offer TailwindCSS classes is provided by the remarkably performant [UnoCSS](https://github.com/unocss/unocss) library. Possibly in a few cases you may see some variation from the [TailwindCSS docs](https://tailwindcss.com/docs/installation) but the Tailwind presets for UnoCSS are very good and parsing performance is fantastic. For more on how to get the most out of UnoCSS with Darkmatter see the [Styling in Darkmatter](./styling.md) section.

## Transformation Approach

When **Darkmatter** finds icons within your markdown content it takes a multi-step approach to transform it:

1. During the Markdown to HTML conversion process Darkmatter's parser will convert the bare HTML tag into a custom component and add in script tag
2.
