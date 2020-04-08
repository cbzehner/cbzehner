+++
title = "Switching to Zola from Hakyll"
date = 2019-04-07
draft = false

[taxonomies]
# tags = ["rust", "web"]
categories = ["programming"]
+++

## Time for a Change

For the past few years, I've served my static site with [GitHub pages](https://pages.github.com/) and generated the source with [Hakyll](https://jaspervdj.be/hakyll). Since it's time for a redesign, why not change the static generator as well?

This post will walk through setting up a new Zola site with a pre-configured theme and deploying it via GitHub Pages, the same way my Hakyll site was deployed.

The old design is a bit long-in-the-tooth.

![Hakyll generated site we'll be replacing](./cbzehner_hakyll.png)

The rest of this article will walk through the steps I used to create this site.

## Setting up Zola

### The Bare Necessities

First, [install Zola](https://www.getzola.org/documentation/getting-started/installation/) on your local machine. I'm running [elementary OS](https://elementary.io/) so I used the snap package `snap install --edge zola`. Now create a fresh Zola installation with `zola init <website-name>` and answer the basic questions, if you're not sure just hit `Enter` to take the default configuration option, it's easy to change later! If you're porting over an existing site, don't worry, we'll cover that once we get the basic structure set up.

Before we go any further, let's ensure our project is set up correctly. Switch to the project directory with `cd <website-name>` and run `zola serve`. Navigate to the provided URL, I got `http://127.0.0.1:1111/` and you should see a `Welcome to Zola!` message asking you to set up an `index.html` file or install a theme.

### Choosing a Theme

We're going to install a theme, but you can [follow the Zola docs](https://www.getzola.org/documentation/getting-started/overview/) to set up a custom site as well. There a couple [different themes to choose from](https://www.getzola.org/themes/) and you [can find many more on GitHub](https://github.com/search?q=zola+filename%3Atheme.toml&type=Code). For my personal website, I will be writing short tutorials and medium length blog posts. I choose [Even](https://github.com/getzola/even), a simple theme that doesn't distract from the content based on a Hugo theme of the same name.

Now follow the [installation instructions from Even](https://github.com/getzola/even#installation). You should create a new file `content/_index.md` that matches the installation instructions and your `config.toml` should now look like:

```TOML
# The URL the site will be built for
base_url = "https://example.com"

# --snip--

theme = "even"

taxonomies = [
  # You can enable/disable RSS
  {name = "categories", rss = true},
  {name = "tags", rss = true},
]

[extra]
# Put all your custom variables here

# The site title show in the header
even_title = "Example Site"

# This is the default menu
even_menu = [
    {url = "$BASE_URL", name = "Home"},
    {url = "$BASE_URL/categories", name = "Categories"},
    {url = "$BASE_URL/tags", name = "Tags"},
    {url = "$BASE_URL/about", name = "About"},
]
```

Now we've got our theme up and running! Try it out with `zola serve`

![Example site with a working theme](./example_site_theme.png)

### Make a Post

Now all that's left is to share our work with the world! Let's update `content/first_post.md` to contain the following:

```Markdown
+++
title = "My first post"
date = 1970-01-01
+++

This is my first post.
```

Try `zola serve` one more time and you should see

![Example site with a single post](./example_first_post.png)

### Copy an Existing Site

If you have an existing static site that you want to preserve, one easy way of doing so is to keep it under version control. I checked out my old site from my git repository to a directory called `cbzehner_hakyll`.

```bash
❯ exa -T -L 2 -a
.
├── cbzehner
│  ├── config.toml
│  ├── content
│  ├── public
│  ├── sass
│  ├── static
│  ├── templates
│  └── themes
├── cbzehner_hakyll
│  ├── .git
│  ├── .gitignore
│  ├── cbzehner-github-io.cabal
│  ├── docs
│  ├── README.md
│  ├── site.hs
│  ├── src
│  ├── stack.yaml
│  └── stack.yaml.lock
```

Copy the `.git/` directory from the old repository to the current project with `copy -r ../cbzehner_hakyll .`. This will preserve the history of the previous static site inside this repository. Update the `.gitignore` file to exclude the `public/` directory where Zola places the build output:

```bash
# Exclude the build directory from version control
public/
```

Now we want to make sure we properly track our theme, which we added as a git submodule. Running `git add .` will generate a warning letting us know to run `git submodule add git@github.com:getzola/even.git themes/even ` which tracks this module as a seperate repository.

### Deploy to GitHub

???


