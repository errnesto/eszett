# _ß_ – eszett  

**Explicit CSS Scopes for React**  

Inspired by [`styled-jsx`](https://github.com/vercel/styled-jsx), but a build-time only plugin, making it compatible with react server components.  

```jsx
import eszett from "eszett";

async function Header() {
  return (
    <div className='header'>
      <h2>Hello World</h2>
      <style href={eszett} precedence="eszett">{`
        .${eszett} {
          &.header {
            background: blue;
          }

          h2& {
            color: white;
          }
        }
      `}</style>
    </header>
  );
}
```

## Installation  

```bash
npm install eszett
```

`eszett` is an [SWC](https://swc.rs/docs/usage/swc-loader) plugin, so it should work wherever SWC works. However, it was primarily developed for use in Next.js projects and has only been tested with Next.js.  

### With Next.js (requires Next.js 15+ and react 19+)  

Add it to your `next.config.js`:  

```js
// next.config.js  
module.exports = {  
  …,  
  experimental: {  
    swcPlugins: [["eszett/swc", {}]],  
  },  
};
``` 

## Usage  

### Scoping Styles to a Component  

When you import `eszett`, it automatically adds a unique class name to your HTML elements (similar to `styled-jsx`). You can then style your component using a `<style>` tag.  

To explicitly scope styles, include the `eszett` variable in your selector:  

```jsx
import eszett from "eszett";

async function Header() {
  return (
    <header className='header'>
      <style>
        .${eszett}.header {
          background-color: blue;
        }
      </style>
    </header>
  )
}
```

Using [CSS Nesting](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_nesting/Using_CSS_nesting) can make your styles more concise:  

```jsx
import eszett from "eszett";

async function Header() {
  return (
    <header className='header'>
      <h2 className='title'>Hello World</h2>
      <style>
        .${eszett} {
          &.header {
            background-color: blue;
          }

          &.title {
            color: white;
          }
        }
      </style>
    </header>
  )
}
```

### Styling Nested Components  

Like `styled-jsx`, `eszett` only adds class names to native HTML elements. It does not automatically add a `className` prop to imported components like `next/link`.  

To scope styles for such components, use the `sz` tagged template literal:  

```jsx
import eszett, { sz } from "eszett";
import Link from "next/link";

async function Component() {
  return (
    <>
      <Link href="/home" className={sz`link`} />
      <style>
        .${eszett}.link {
          color: red;
        }
      </style>
    </>
  );
}
```

### Styling Children  

Since scoping must be explicitly applied, omitting the `eszett` scope class will make selectors global. This allows components to apply styles to all children without having access to their class name props.  

```jsx
import eszett from "eszett";

async function Component(props) {
  return (
    <div className="wrapper">
      {props.children}
      <style>
        .${eszett}.wrapper strong {
          color: red;
        }
      </style>
    </div>
  );
}
```

### Deduplicating Styles (requires React 19)  

`eszett` leverages [React's built-in style deduplication](https://react.dev/reference/react-dom/components/style#special-rendering-behavior) for `<style>` tags with a `href` and `precedence` attribute. Use the `eszett` variable as the `href` to enable this behavior. **This means you can only have one single style tag per component – otherwise you need to use different href attributes per style**

```jsx
import eszett from "eszett";

async function Header() {
  return (
    <div className="header">
      <style href={eszett} precedence="eszett">{`
        .${eszett}.header {
          background: blue;
        }
      `}</style>
    </div>
  );
}
```

## How it Works  

`eszett` generates a unique ID for each React component, rewrites the `className` values, and provides helper methods to access the scope name.  

### Rewriting classNames  

HTML elements receive unique `className` values by combining the generated scope name and the original class names.  

```jsx
import "eszett";

<div className="header" />;

// Transformed into:

<div className={"23u00ds-1" + " " + "header"} />;
```

### Accessing the Scope Name  

The `eszett` variable provides access to the unique scope name, which is derived from hashing the file path and incrementing a counter for each top-level function.  

```js
import eszett from "eszett";

console.log(eszett);

// Transformed into:

console.log("23u00ds-1");
```

### Tagged Template Literal  

The `sz` helper applies the scoped class name to components that do not automatically receive one.  

```jsx
import { sz } from "eszett";

<Link className={sz`header`} />;

// Transformed into:

<Link className={"23u00ds-1" + " " + `header`} />;
```

## Plans

While I like the explicitness of the scoping, in practice it’s easy to accidentally create global selectors. In the future, I plan to enhance eszett by automatically adding the scope and introducing a `:global()` selector to opt out of scoping, making the process even simpler.

## Why eszett?  

I wanted a name that could be shortened to two letters (sz), representing "scoped CSS." I did not like my initial ideas, `sc` or `sx`, but then I thought of the German letter ß (eszett), which, to me, could be what it sounds like when you scramble "scope", "css" and "styles" together. That’s how I landed on the name.

Think of it as "scoped stylez"!

## Heavily Inspired by [styled-jsx](https://github.com/vercel/styled-jsx)

I’ve always appreciated the developer experience (DX) and core concept of `styled-jsx`: it simplifies styling by simply adding a class name to elements. However, I often wished for direct access to the class name it generates. This plugin was born from that idea.  

The decision to create `eszett` solidified when I tried using `styled-jsx` with React Server Components and found it lacking compatibility.  

It’s not just the ideas that are inspired by `styled-jsx` — I also reused parts of its implementation. Specifically, I adapted the logic for adding class names and even some of its tests.  

