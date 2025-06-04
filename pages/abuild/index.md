# abuild

> a tool for building

## structure ![feature]

> <u>w</u>orkspace:
> > .abuild:
>
> > pro<u>j</u>ect A:
> >
> > > .abuild:
> >
> > > src:
> >
> > > rc:
> >
> > > build:
> > > > <u>p</u>rofile A:
> > > > > bin
> > >
> > > > <u>p</u>rofile B:
> > > > > bin
>
> > pro<u>j</u>ect B:
> > > ...

## commands ![feature]

> > ![feature] init/create/remove workspace
> > + ![note] init/create: The workspace directory must be empty.
> > ```shell
> > $ abuild init (-w|--workspace)
> > workspace "<current_directory>" was initialized successfully.
> > $ abuild create (-w|--workspace) <workspace_name>
> > workspace "<workspace_name>" was created successfully.
> > $ abuild remove (-w|--workspace) <workspace_name>
> > workspace "<workspace_name>" was removed successfully.
> > ```
>
> > ![feature] undo/redo
> > ```shell
> > $ abuild undo
> > the last operation is '<last_operation>'
> > ... # output of the undo operation
> > $ abuild redo
> > the last operation is '<last_operation>'
> > ... # output of the redo operation
> > ```
>
> > ![feature] init/create/remove project (workspace|none)
> > + ![note] The current folder must be a workspace, or the -w option must be provided.
> > ```shell
> > $ abuild init (-j|--project) ((-w|--workspace) <workspace_path>)?
> > project "<current_directory>" was initialized successfully.
> > it is added to workspace '<current_workspace>'.
> > $ abuild create (-j|--project) <project_name> ((-w|--workspace) <workspace_path>)?
> > project "<project_name>" was created successfully.
> > it is added to workspace '<current_workspace>'.
> > $ abuild remove (-j|--project) <project_name> ((-w|--workspace) <workspace_path>)?
> > project "<project_name>" was removed successfully.
> > it is removed from workspace '<current_workspace>'.
> > ```
>
> > ![feature] create/remove profile (workspace|project)
> > + ![note] The current folder must be a (workspace|project), or the (-w|-j) option must be provided.
> > ```shell
> > $ abuild create (-p|--profile) <profile_name> ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
> > profile "<profile_name>" was created successfully.
> > it is added to (project '<current_project>'|workspace '<current_workspace>').
> > $ abuild remove (-p|--profile) <profile_name> ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
> > profile "<profile_name>" was removed successfully.
> > it is removed from (project '<current_project>'|workspace '<current_workspace>').
> > ```
>
> > ![feature] build/clean (workspace|project|profile)
> > ```shell
> > $ abuild build ((-p|--profile) <profile_name>)? ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
> > building...
> > ... # output of the build process
> > building finished.
> > $ abuild clean ((-p|--profile) <profile_name>)? ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
> > cleaning...
> > ... # output of the clean process
> > cleaning finished.
> > ```
>
> > ![feature] run (workspace|project|profile)
> > ```shell
> > $ abuild run ((-p|--profile) <profile_name>)? ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
> > ... # output of the build process (if not already built)
> > running...
> > ... # output of the run process
> > the program is exited with code '<exit_code>'.
> > ```
>
> > ![feature] rebuild = clean \& build (workspace|project|profile)
> > ```shell
> > $ abuild rebuild ((-p|--profile) <profile_name>)? ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
> > ... # output of the clean process
> > ... # output of the build process
> > ```

## languages ![feature]

> rust ![feature]

> c/c++ ![feature]
>
> > make tool:
> >
> > > ![feature] make(Makefile)
> > >
> >
> > > ![feature] cmake(CMakeLists.txt)
> > >
> >
> > > ![feature] msbuild(*.sln)

> c# ![feature]
>
> > make tool:
> >
> > > ![feature] make(Makefile)
> > >
> >
> > > ![feature] msbuild(*.csproj)

---

[note]: https://img.shields.io/badge/note-orange.svg?color=ddbb00

[bug]: https://img.shields.io/badge/bug-red.svg

[feature]: https://img.shields.io/badge/feature-orange.svg
