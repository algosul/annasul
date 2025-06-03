# project annasul

---

## [abuild](./abuild) ![feature]

### structure ![feature]

> workspace:
> > .abuild:
>
> > project A:
> > > .abuild:
> >
> > > src:
> >
> > > rc:
> >
> > > build:
> > > > profile A:
> > > > > bin
> > >
> > > > profile B:
> > > > > bin
>
> > project B:
> > > ...

### commands ![feature]

> > ![feature] create/remove workspace
>
> > ![feature] undo/redo
>
> > ![feature] create/remove project (workspace|none)
>
> > ![feature] add/remove profile (workspace|project)
>
> > ![feature] build/clean (workspace|project|profile)
>
> > ![feature] run (workspace|project|profile)
>
> > ![feature] rebuild = clean & build (workspace|project|profile)
>
> > ![feature] create/remove profile (workspace|project)

### languages ![feature]

> rust ![feature]

> c/c++ ![feature]
> > make tool:
> > > ![feature] make(Makefile)
> >
> > > ![feature] CMake(CMakeLists.txt)
> >
> > > ![feature] msbuild(*.sln)

> c# ![feature]
> > make tool:
> > > ![feature] make(Makefile)
> >
> > > ![feature] msbuild(*.csproj)

---

## [annasul](./annasul) ![feature]

---

[bug]: ./badges/bug.svg

[feature]: ./badges/feature.svg
