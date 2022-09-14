# Darkmatter Features

This section describes a set of built-in transformation features which users can opt-in/out of:

- github style tables
- github task list styling
- markdown footnotes
- code highlighting (including line highlighting)
- code block imports

    You can import code snippets from other files with a Vuepress derived style syntax:

    ```md[](https://vuepress.vuejs.org/guide/basic-config.html)
    Here is some code I wrote:

    <<< ./my-code.ts
    ```

- link tagging

    Provides automatic class tagging of links based on type:

  - `anchor-tag`
  - `local-link`
  - `external-link`
  - `email-link`
  - etc.

- meta tagging

    Allows simple and intuitive mapping of Frontmatter into both router metadata as well as HTML `<meta>` and `<title>` tags.

- emoji expansion

    Inline references such as `:smile:` will be converted to the emoji equivalent unless this feature is turned off.

- expandable lists

    A markdown list can be easily made _expandable_ to conserve vertical space and/or help to focus readers on key content while allowing drill-downs where readers wants to.

    Defaults can be defined in config but page level control and overrides is also provided:

    ```md
    * foo
      - f1
      - f2
    + bar
      - b1
      - b2
    - baz
      - ba1
      - ba2
    ```

    > in the example above:
    >
    > - the `foo` sub-items will start collapsed,
    > - the `bar` sub-items will start expanded,
    > - the `baz` sub-items will be shown but not be expandable/contractable (or whatever is defined in config as default behavior)

    &nbsp;
- list visualization (future)

    Allows lists of data to be treated as data for a data-visualization component:

    ```md
    This is an example of data visualization supported out of the box:

    ::: visualize as bar-chart
    - red: 15
    - blue: 10
    - green: 55
    :::
    ```

- visualize external data (future)

    While simple data samples often fit the convenient inline list format, you can also reach into other data (both local to your build as well as external):

    ```md
    There is a lot of data in the world:

    <<< visualize as bar-chart from "./data.json"

    but not just local data, also stuff from that big world we live in:

    <<< visualize as bar-chart from "http://big.world/data"
    ```
