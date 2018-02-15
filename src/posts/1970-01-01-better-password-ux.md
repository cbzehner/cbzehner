---
title: Better "Incorrect Password" UX
---

One thing I dislike about the modern email/password system most sites use as authentication, when you make a tiny mistake and they *clear the password field*. Why!?! I just put it in, don't make me type it in all over again. Just let me hit show and edit the field.

Why do I have to put in my email every time? For that matter, why do I put in my email and password on the same screen?

```
| ----------------------- |
| Welcome to WebsiteCo!   |
|                         |
|   -----------           |
|  |    email  |          |
|   -----------           |
|                         |
|   -----------           |
|  | password  |          |
|   -----------           |
|                         |
|     Sign In             |
|                         |
|    (Sign Up?)           |
| ----------------------- |
```

should be

```
| ----------------------- |
| Welcome to WebsiteCo!   |
|                         |
|   -----------           |
|  |    email  |          |
|   -----------           |
|                         |
| ----------------------- |
```

then either sign me up:

```
| ----------------------- |
| Welcome to WebsiteCo!   |
|                         |
|Is this your first visit?|
|                         |
|   Sign Up!              |
|   -----------           |
|  | password  |          |
|   -----------           |
|                         |
| ----------------------- |
```

or sign me in:

```
| ----------------------- |
| Welcome to WebsiteCo!   |
|                         |
|  Thanks for returning!  |
|                         |
|   Sign In!              |
|   -----------           |
|  | password  |          |
|   -----------           |
|                         |
| ----------------------- |
```

PS - use Lastpass! Really!
