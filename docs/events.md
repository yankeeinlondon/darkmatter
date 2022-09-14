# Darkmatter Event Hooks

An _event hook_ in Darkmatter is a set of timeframes in the transformation pipeline in which a client/project can provide input into the transformation.

## Hook Types

Defining a hook takes one of three forms (or even a hybrid of these three):

1. a static value
2. a `RegExp` matcher
3. a callback function

A static value often is all you need but it can be a limiting factor too as you can not _dynamically_ respond to an individual document which you are processing. With that in mind let's discuss the two dynamic variants and why you might choose one over the other.

### The `RegExp` matcher

Events allow you to pass in responses as a dictionary object where the keys of the dictionary are regular expressions and the values are _potential values_ which will be assigned to a property if the RegExp passes (aka, the first one to match).

So in a simple example imagine that I have the following tuples defined:

```ts
const t1 = ["I love bananas", "bananas"];
const t2 = ["my favorite fruit is (\w+)", "$1"];
const t3 = ["i love (wine|grapes)", "grapes"];
```

We can now _hook_ into the **`frontmatter_defaults`** hook with the following added to our configuration:

```ts
import { Options } from "darkmatter";
const options: Options = {
    frontmatter: {
        defaultValues: {
            fruit: {
                regex: [t1,t2,t3],
                fallback: "rotten apples"
            }
        }
    }
}
```

Now if we find "I love bananas" within a page the default **frontmatter** property "fruit" will be "bananas". If the text "i love wine" is found but _neither_ of the other two RegExp's are found first, then the fruit will be set to "grapes". The key to understanding this is that _only_ the first match will be considered and the RegExp will be applied to "body" of the page.

Sometimes, however, the logic you're wanting to employ does not relate to the body of the page but rather the file's path/route, or another frontmatter or darkmatter property. This too is supported by using the `regex_path`, `regex_fm_[prop]` or `regex_dm_[prop]` in your definition. For example, should I want to base the "category" property based on the document's route, I might do it like so:

```ts
const options: Options = {
    frontmatter: {
        overrides: {
            category: {
                regex_path: [tp1, tp2, tp3],
                fallback: "unknown"
            }
        }
    }
}
```

> Note: in the case of targeting _frontmatter_ or _darkmatter_ just be sure to include the property name you want to target.

Be aware that a given property _can_ use multiple regex targets but they will always follow a static order -- path, body, frontmatter, darkmatter -- and the first match across any of them will be used.

This style of using a dynamic value in your hooks is highly performant and solves many use cases but more advanced use cases will not be achievable. This is where you'll need to consider callbacks. In general you may find the callback approach better for even those situations where the RegExp matcher would suffice but there is typically going to be a performance penalty for this as callbacks require inter-process communication.

## Event Lifecycle

The following events are made available as part of the configuration hash a project must pass in during activation.

1. `frontmatter_defaults`

   Set default values for the key/value pairs on a give page; if the page has a value defined explicitly then the page's value will be overridden. This hook _does not_ allow for the use of _darkmatter_ properties as they have not been calculated yet.

2. `shortcodes`

    Short codes are a dictionary of find/replace values which are substituted into the markdown text with no regard to _where_ in the document they may land. For this reason, it is generally a good idea to name shortcode keys using a convention that avoids accidental replacements.

3. `frontmatter_overrides`

    Similar to `frontmatter_defaults` but with two key differences:

    1. the `darkmatter` properties _are_ available as an input
    2. the values set in this hook are the final value and _override_ any potential value found on the page

    > Note: using both the callback and RegExp matcher patterns you can target the current value of the property and use that as an input

4. `code_block`

    All code blocks can be analyzed and mutated as seen fit.

    > Note: if the user has turned on `highlight_code` then this hook will receive the code _before_ it has been tokenized for code highlighting.

5. `code_block_formatted`

    All code blocks -- after having been tokenized for code highlighting -- can be hooked into here.

    > Note: if the user has _not_ turned on `highlight_code` then this event will never fire.

6. `inline_code`

    The same as `code_block` but for _inline_ code.

7. `inline_code_formatted`

    The same as `code_block_formatted` but for _inline_ code.

8. `list_block`

    Each list block containing ordered or unordered list items.

9. `list_item`

    Each list item in a list block.

10. `quotation`

11. `table`

    Provides access to all `<table>` blocks.

12. `th`

    Provides access to all the `<th>` blocks in a table.

13. `tr`

    Provides access to all `<tr>` blocks in a table.

14. `td`

    Provides access to all the `<td>` blocks in a table.

15. `heading`

    Provides all `<h1> ... <h6>` tags.

16. `h1`, `h2`, `h3`, `h4`, `h5`, and `h6`

    Provides _specific_ heading blocks.

17. `footnote`
18. `strikethrough`

    Provides all strikethrough text on page by block. This event is only fired if the `useStrikethrough` feature is set.

19. `emoji`

    Provides all emoji symbols found on page. An example hook might look like:

    ```ts
    {
        class: "",
        content: "star_struck",
        ops: EventOps
    }
    ```

20. `slot`

    All slot blocks passed into hook.

    ```md
    This is a page.
    :::left { class="text-sm" }
    but some of it should be placed in the "left" slot
    :::
    ```

    In this example the hook receives:

    ```ts
    {
        class: "text-sm",
        name: "left",
        content: "but some of it should be placed in the \"left\" slot",
        hasChildren: false,
        ops: EventOps
    }
    ```

21. `toc`

    Provides the full table of contents once all headings are parsed.
