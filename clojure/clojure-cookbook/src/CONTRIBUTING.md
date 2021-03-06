# Contributing to Clojure Cookbook

Our goal with *Clojure Cookbook* is to build the best possible cookbook, chock full of excellent Clojure recipes. To do this we need the help of the whole community. From typos and factual corrections, to ideas for recipes, to whole recipes themselves, your experience is what we need to build the best *Clojure Cookbook*.

## Valuable Contributions

Nearly any knowledge or bit of information you could contribute to the cookbook is valuable, but here are some concrete ideas:

* Suggest an idea by [creating an issue](../../issues/new?title=Idea:%20I've%20got%20this%20great%20idea...&body=So here's what I was thinking...&labels[]=idea) with the tag "idea".
* Correct a typo or error in an existing recipe.
* Proofread and provide feedback on existing recipes.
* Write a recipe yourself (find more details in the [Recipes](#recipes) section below.)

*Major contributions such as a whole recipe require you to [license](#licensing) your submission. However, you can submit pull requests for typos, factual corrections and ideas without one*

## Recipes

One of the most helpful contributions you could make to the cookbook is a complete recipe (anywhere from 1-10 pages, usually; the length of a typical blog post.) This section provides an overview of what a recipe looks like, where to draw ideas for recipes from and how to properly license your content so that it may be published in the final book.

### The format

The general format for any cookbook recipes is **Problem**, **Solution** and **Discussion**. The length of a recipe is generally one to two pages, but there is no hard rule on length (big or small.) Simple examples may only warrant half a page, and more complex examples might push three or four pages. If you're in doubt, inquire early.

#### The *literal* Format

Recipes in *Clojure Cookbook* should be written in [AsciiDoc](http://www.methods.co.nz/asciidoc/) a markup language very similar to Markdown, but much more well suited for writing books. AsciiDoc has support for advanced features like [callouts](http://www.methods.co.nz/asciidoc/asciidoc.css-embedded.html#X105), footnotes, [cross referencing links](http://www.methods.co.nz/asciidoc/asciidoc.css-embedded.html#_internal_cross_references) and most importantly generating indices for the final book.

The `conventions.asciidoc` document serves as the guidelines for how
to convey different things in AsciiDoc, like source code blocks, REPL
sessions, shell sessions, expression results, output, etc.. Please
read it carefully to learn about these important elements of good
recipes.

You can find more information on the format in the [AsciiDoc User Guide](http://www.methods.co.nz/asciidoc/asciidoc.css-embedded.html) or across numerous pre-existing `*.asciidoc` files in this repo.

#### Recipe File Organization

Recipes must reside in their own directories under the relevant chapter or section, or in the `kitchen-sink` directory if you are not sure what chapter it belongs in, e.g.:

```
primitive-data/math/random-numbers/random-numbers.asciidoc
```

Any included images or extended source code examples should reside in this directory as well.

#### Problem

A problem statement is a short description of the problem the recipe provides a solution to. Problems should be _statements_ focused around the phrase "you want to". More complicated problems may break from this convention, but you should strive to reduce every problem to the simplest "you want to" statement possible.

**Appropriate examples**:

```
You want to generate a lazy sequence covering a range of dates and/or times.
```

```
You want to create a list data structure in your source code.
```

Or, a longer example:

```
Normally, maps are strictly one value per key; if you +assoc+ an
existing key, the old value is replaced.

Sometimes it is useful to have a map-like interface (a "multimap")
which is capable of storing _multiple_ values for the same key. There
is no such capability built in to Clojure.

So, how would you design a multimap data structure, backed by a normal
map?
```

#### Solution

The solution to a problem briefly introduces the approach and contains the code for the actual *solution* to the problem.

Solutions should be centered around _imperative_ sentences which plainly tell the user what to do, followed by the smallest small code sample necessary to illustrate the approach. Tell the user what to do, then show them.

**The solution to a problem should *discuss* the solution as little as possible** ??? leave this for the Discussion section.

See the following recipes for representative examples:

##### Examples

* [Performing Find and Replace on Strings](primitive-data/strings/find-and-replace/find-and-replace.asciidoc)
* [Performing Fuzzy Comparison](primitive-data/math/fuzzy-comparison/fuzzy-comparison.asciidoc)
* [Adding or Removing Items from Sets](composite-data/sets/adding-and-removing/adding-and-removing.asciidoc)

#### Discussion

The Discussion section is where your exposition occurs. Why would you solve this problem this way? What (if any) alternatives are there? How do the functions and methods work in depth ??? how and when would you use them in different ways? It is perfectly appropriate to include further code samples or examples in the discussion section to further illustrate the recipe.

Use admonition paragraphs to highlight helpful side notes (with "TIP:") or potentially dangerous side effects (with "WARNING:"). See the Asciidoc docs for more info on admonitions.

The [Examples](#examples) from the Solution section above are representative of Discussion sections.

### Style

This is a book in the O'Reilly _Cookbook_ series. When in doubt, don't be afraid to refer to other _Cookbook_ titles for ideas on topics, style, structure, etc..

The general *tone* of writing in the book should phrase things from the perspective of the reader. For example, use **"You should use X technique to accomplish Y"** instead of **"I would use X to accomplish Y"**.

When instructing readers what technique or library to use make it clear whether something is the only option, the best option or one of many reasonable options. In particular, be mindful of the differences between words like *should*, *may*, etc ??? where possible adhere to [RFC2119](http://www.ietf.org/rfc/rfc2119.txt)'s recommended usage of such terms.

### Licensing

The full book itself is licensed under a [Creative Commons Attribution-NonCommercial-NoDerivs](http://creativecommons.org/licenses/by-nc-nd/3.0/) license, meaning that it can be freely distributed but not modified or resold commercially. This is to ensure the book is always of good quality (as befits the O'Reilly name) and to prevent third parties from profiting commercially by redistributing something intended to be free. Technically speaking, this book is open source in the "free as in beer" sense, rather than "free as in speech."

However, in order for us to accept submissions, those submissions must be licensed such that *we* (the authors and O'Reilly) *can* in fact use your submission to create derivative works, sell print copies, and distribute it under a more restrictive license (discussed above.)

Therefore, all sizable public contributions (anything more than a typo correction) must be licensed under the license found [here](.license-assignments/template.md). The easiest way to do this is by placing a copy of [.license-assignments/template.md](.license-assignments/template.md) in the [.license-assignments/](.license-assignments) folder with the name `<your github username>.md` the very first time you make a contribution.

Source files should contain comments indicating the name of the author for which you want to receive attribution.

If you have any questions or concerns about the licensing model, please don't hesitate to ask us.

### Ideas

You'll find a list of ideas under the [ideas tag](../../issues?labels=idea&milestone=&page=1&state=open) of our GitHub Issues page. Feel free to submit your own ideas with a title like: `Idea: recipe covering http-kit` or `Idea: Creating a leiningen plugin`.
