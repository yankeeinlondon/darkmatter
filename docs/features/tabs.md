# Tab Menus

```md
# My Cool Tabbed Nav

::tabs
::tab "One"
This is the first tab which we called "One"

::tab name="Two", inline="./sub-page2.md"
::tab "Three", route="./sub-page3.md"

::end-tabs
```

- tabs menu rendered using just Markdown
- tab content can just be included inside the same MD file (aka, `One`)
- alternatively content can be _inlined_ at build time, or it can be brought in only at run-time when that tab is selected by treating this as a sub-route to the containing page.

## Properties

1. `theme` - comes with a default `light` and `dark` theme but you can create your own. When setting themes you should use a tuple value to indicate how to handle the light vs dark mode but if you only specify one theme it will be used for both modes

   ```md
   ::tabs theme=light,dark
   ```

2. `color` - the

### Menu Level

### Tab Level
