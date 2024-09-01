# *ß* – eszett

Explicit css scopes for react – by using a unique css class per component generated at build time.

# TODO
- [ ] update imports to prioritize scopeName and match new readme
- [ ] dont transform classNames in files that don't import sz
- [ ] maybe add a custom style component

```jsx
import eszett from "eszett";

function Header() {
  return (
    <header>
      <h2>Hello World</h2>
      <p>Subtitle</p>

      <style href={eszett}>{`
        .${eszett} {
          &.header {
            background: blue;
          }

          &.title {
            color: white;
          }

          &.header p {
            color: grey;
          }
        }
      `}</style>
    </header>
  );
}
```

## Install

```
npm install eszett
```

`eszett` is an [swc](https://swc.rs/docs/usage/swc-loader) plugin – so it should work wherever swc works.

### With NextJs

In nextjs you can add it to your `next.config.js`:

```js
// next.config.js
module.exports = {
  …
  experimental: {
    swcPlugins: [["eszett/swc", {}]]
  }
}
```

## Usage

### How it Works

eszett generates a unique id for each react component and gives you two helper methods to use it:

#### Rewriting classNames:

```jsx
// this input
import "eszett";
<div className='header' />;

// will be tranformed to:
<div className={'23u00ds-1' + ' ' + 'header'} />;
```

#### Access the scope name as variable

```js
// this input
import eszett from "eszett";
console.log(eszett);

// will be transfomed to to:
console.log("23u00ds-1");
```

#### `sz` tagged template literal

```jsx
// this input
import { sz } from "eszett";
<Link className={sz`header`} />;

// will be tranformed to:
<Link className={'23u00ds-1' + ' ' + `header`} />;
```

> The eszett scope name is generated by hashing the file path of the component and incrementing a counter
> for each top level function in each file

Together with support for [`<style>` tags in react 19](https://react.dev/reference/react-dom/components/style) and [css nesting](https://developer.mozilla.org/en-US/docs/Web/CSS/Nesting_selector) this is all we need to encapsulate our styles inside our components.

### Without modern css

If you need to suport older Browsers you could use something like [postcss-preset-env](https://preset-env.cssdb.org/features/#nesting-rules) or you can just write classic css:

```jsx
import eszett from "eszett";

function Header() {
  return (
    <header className='header'>
      <h2 className='title'>Hello World</h2>
      <p>Subtitle</p>

      <style href={eszett}>{`
        ${eszett}.header {
          background: blue;
        }

        ${eszett}.title {
          color: white;
        }

        ${eszett}.header p {
          color: grey;
        }
      `}</style>
    </header>
  );
}
```

### Styling children

classNames are only rewritten for native html elements and locally defined elements.
If you want to pass the scoped class name down to other components, you can use the `sz` template literal to do that:

```jsx
import eszett, { sz } from "eszett";

function PassClassNameToChildren() {
  return (
    <>
      <Link className={sz`link`} href="/home">
        Home
      </Link>
      <style href={eszett}>{`
        ${eszett}.link {
          color: red;
        }
      `}</style>
    </>
  );
}
```
