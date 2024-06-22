# ß – eszett

Scoped css for react component at build time by using a unique class name per component.

```jsx
import sz, { scopeName } from "eszett";

function WithCSSNesting() {
  return (
    <header className={sz`header`}>
      <h2 className={sz`title`}>Hello World</h2>
      <p>Subtitle</p>

      <style href={scopeName}>{`
        .${scopeName} {
          &.header {
            background: blue;
          }

          &.title {
            color: white;
          }

          & p {
            color: grey;
          }
        }
      `}</style>
    </header>
  );
}
```

eszett generates a unique id for each react component and gives you two helper methods to use it:

### `sz` tagged template literal

```js
// this input
import sz from "eszett";
const className = sz`header`;

// will be tranformed to:
const className = "23u00ds-1 " + `header`;
```

### `scopeName` Direct access to the generated scope name

```js
// this input
import { scopeName } from "eszett";
console.log(scopeName);

// will be transfomed to to:
console.log("23u00ds-1");
```

## Usage

Together with support for [`<style>` tags in react 19](https://react.dev/reference/react-dom/components/style) and modern css features such as [nesting](https://developer.mozilla.org/en-US/docs/Web/CSS/Nesting_selector) or even [scope](https://developer.mozilla.org/en-US/docs/Web/CSS/@scope) this allows us to write scoped css inside our components.

### With `@scope`

```jsx
import sz, { scopeName } from "eszett";

function WithCSSScope() {
  return (
    <header className={sz`header`}>
      <h2 className={title}>Hello World</h2>
      <p>Subtitle</p>
      <style href={scopeName}>{`
        @scope ${scopeName}.header {
          :scope {
            background: blue;
          }

          .title {
            color: white;
          }

          p {
            color: grey;
          }
        }
      `}</style>
    </header>
  );
}
```

### Without modern css

```jsx
import sz, { scopeName } from "eszett";

function WithoutModernCss() {
  return (
    <header className={sz`header`}>
      <h2 className={title}>Hello World</h2>
      <p>Subtitle</p>
      <style href={scopeName}>{`
        ${scopeName}.header {
          background: blue;
        }

        ${scopeName}.title {
          color: white;
        }

        ${scopeName} p {
          color: grey;
        }
      `}</style>
    </header>
  );
}
```

### Styling children

Since you activly need to add the scope to the classes, you can style children,
as you would with plain css, but if you want to pass the scoped classed down you can also do that:

```jsx
import sz, { scopeName } from "eszett";

function PassClassNameToChildren() {
  return (
    <>
      <Link className={sz`link`} href='/home'>Home</Link>
      <style href={scopeName}>{`
        ${scopeName}.link {
          color: red;
        }
      `}</style>
    </>
  );
}
```
