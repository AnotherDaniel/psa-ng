# Three stages of quality process application

Each stage of a TSF-enabled process as envisioned by `tsftemplate` comprises a set of activities and related tools, and that are described in greater detail below. By providing this starting point, `tsftemplate` wants to provide inspiration about how the approach might be streamlined and automated as much as possible.

## Stage I

Stage I is about defining how a quality method should be applied to an actual project/codebase, and getting set up to integrate the method into regular development workflows as quickly as possible. TSF is designed to grow in scope over time, following the focus areas identified by the project team. Therefore in an Open Source context it is a prime goal to lower the bar to entry, even at the expense of coverage or specificity - that can always be evolved going forward.

Combined with the capability for tool-based manipulation of the TSF statement graph, this offers a unique opportunity: community-driven TSF statement libraries, that build on top of the core TSF tenets (and each other), and provide technology- or context-specific tailoring blocks. For example:

- The Rust programming language comes with extensive language- and compiler-level security features, as well as a very rich tooling ecosystem that extends to managing licensing declarations or built-in CVE tracking and notification across the entire dependency tree: this is an ideal foundation for a Rust-TSF-library, that any Rust project can pull in to get started with applying TSF.
- The Eclipse Foundation defines a comprehensive set of [process](https://www.eclipse.org/projects/dev_process/ "Eclipse Foundation Development Process") and [behavioral guidance](https://www.eclipse.org/projects/handbook/ "Eclipse Foundation Project Handbook") that every Eclipse Foundation Open Source project must adhere to. Again an ideal starting point for EF projects, which can get ahead on TSF application by simply pulling in a Eclipse-TSF-library that codifies the Eclipse-related processes and best practices.

The creation and refinement of such statement libraries would make an ideal TSF community effort, where statement sets could be maintained in a contribution-centric model, allowing the open sharing of best practices for each relevant technology stack or process context.

The TSF library scenario, as illustrated in `tsftemplate`, is shown in the following diagram:

![Lower the bar to entry via community-driven TSF statement libraries](img/tsftemplate-Stage%20I.drawio.svg "tsftemplate Stage I")

***Note***: At the moment, the TSF `trudag` tool does not yet fully support this vision, and there isn't yet a community-contrib model for TSF statement libraries. Therefore, `tsftemplate` operates from a starting point where TSF core tenets plus an Eclipse Process library have been pre-imported into the project TSF graph as starting points.

### Stage I - Workflow

1. At project start/when introducing TSF to the project, all relevant TSF context libraries are identified from the TSF contrib repository.
2. the project TSF graph is initialized by importing TSF core tenets, plus any contrib statement libraries and their potential dependencies.
3. If at any point in the project lifecycle there is a major change to used technology or applied process where a TSF contrib library exists, that could be added into the project graph as necessary.

### Stage I - Associated tools, related project files

The main tool for manipulating the TSF graph is the [TSF `trudag` tool](https://pages.eclipse.dev/eclipse/tsf/tsf/tools/install.html). `tsftemplate` might provide a devcontainer in the future that comes with `trudag` preinstalled; the main use of the tool happens inside the `tsflink` GitHub action used by `tsftemplate` CI.

- `.dotstop.dot`: this is the DOT graph metadata file containing TSF graph nodes, links and freshness hash data.
- `trustable/upstream`: this is the upstream TSF statements imported into `tsftemplate`, comprising the core TSF tenets and an Eclipse Process TSF statement library.
- `trustable/tsftemplate`: this folder contains all `tsftemplate` project specific TSF statements - refer to the [next section](#stage-ii) for more information.

## Stage II

Once a quality method is selected and initially tailored, the main part of method application begins: the project needs to make specific statements about how it supports demands and guidelines expected by the tailored method (compare the [previous section](#stage-i)). For instance: where the quality method requires test automation, the specific project will explain (via a dedicated statement in its TSF graph, linking to the more generic parent statement) that it uses GitHub action pipelines for any PR merge and release build. Once this conceptual linking is done, the final step is to provide proof of one's assertions - in the example, this might be done by pointing to the CI configuration ruleset that is also part of the repository.

This work is by far the largest part of quality method application, and it is a continual effort that accompanies the development work similar to the writing of test cases or the updating of documentation.
Any and every part of the development work can become the object of TSF statements and supporting evidence, depending on context and desired outcome - code, documentation, specification, configuration, collected performance data, release artifacts like test reports or coverage metrics, and so on.

![Help devs tie together code, docs, specs, CI and TSF monitoring - once](img/tsftemplate-Stage%20II.drawio.svg "tsftemplate Stage II")

The first activity - the extension of the TSF graph with project-specific statements - is supported by the `trudag` tool, and the fact that all content created that way is expressed as markdown snippets that are managed alongside the project code and all other files.

`tsftemplate` aims to showcase how the second activity - the linking of statement-supporting evidence - can be done in a 'don't repeat yourself' way, such that developers only need to formally declare the relationship between some project artifact and the statement it supports as part of the project CI, to then have both static (eg. code files) and dynamic (eg. release test reports) artifacts pulled into TSF evaluation, scoring and reporting as part of project automation.

### Stage II - Workflow

1. For TSF areas that the project wants to improve its scoring in, developers consider how they support the TSF goals (compare [Stage I](#stage-i)); from these considerations, project-specific TSF statements are added (via `trudag`) describing attributes, activities or measures performed by the project to support the stated goal.
2. Each project TSF statement requires supporting evidence to positively contribute to the overall TSF score; therefore developers now implement whatever mechanisms are necessary to provide such evidence, and create links between evidence and supported statement in the project CI using the `tsffer` GitHub action.

### Stage II - Associated tools, related project files

The main tool for manipulating the TSF graph is the [TSF `trudag` tool](https://pages.eclipse.dev/eclipse/tsf/tsf/tools/install.html). For automating the linking of evidence to project TSF statements, `tsftemplate` uses the [`tsffer` ("TSF transfer")](https://github.com/AnotherDaniel/tsffer) GitHub action. Tsffer action steps simply are provided with the evidence-information and targeted TSF statement IDs, and will during a workflow run record these relationships in little metadata files, which in [Stage III](#stage-iii) get pulled into TSF scoring and report generation.

- `trustable/tsftemplate`: this folder contains all `tsftemplate` project specific TSF statements - building on and linking back to the more generic initial TSF graph as composed in [Stage I](#stage-i).
- `.github/workflows/release.yaml`: this is the `tsftemplate` release workflow, providing some examples of how `tsffer` is used for recording evidence-statement links during a release run.
- `docs/`: includes a simply requirements specification for `tsftemplate`, for showcasing requirements tracing using [OpenFastTrace](https://github.com/itsallcode/openfasttrace).

## Stage III

The goal of stage III is to make maximum use of all the work put into stages I and II - all the quality documentation, TSF elaboration and identification of supporting evidence should enable automated creation of quality documentation and scoring. Therefore `tsftemplate` comes with a CI workflow that continually delivers TSF quality reporting and computes the trustability score, based on the state of the TSF graph and related evidence that is collected during e.g. a release run.

![Enable CI workflows that continually score and deliver TSF quality reporting](img/tsftemplate-Stage%20III.drawio.svg "tsftemplate Stage III")

In Stage III, `tsftemplate` makes use of existing tools and GitHub capabilities to tie together a simple release pipeline which offers all of the above capabilities:

- The TSF tool itself is capable of scoring the status of the projects TSF graph, and publish a quality report that is ready to be served as a `mkdocs` website.
- All the statement breakdown and evidence linking work is happening as part of the continuous project development work (compare  [Stage II](#stage-ii)).
- TSF `trudag` comes with a plugin concept for adding custom evidence report renderers; `tsftemplate` uses this showcase integration of [OpenFastTrace (OFT)](https://github.com/itsallcode/openfasttrace)-managed requirements, and how such requirement traces could become part of the quality report evidence.
- (`trudag` also provides the concept of evidence validation (again based on a plugin API); at the moment `tsftemplate` does not yet make us of that feature.)
- Generated quality data can be used as parts of dashboards or similar visualization - `tsftemplate` uses the [shields.io](https://shields.io/) API to include last release's TSF score and report link at the top of the project README (this file).
- The `trudag`-generated quality report is embedded in a [Material-for-mkdocs](https://squidfunk.github.io/mkdocs-material/) website, and served via GitHub pages - all automated as part of the `tsftemplate` release workflow.

### Stage III - Workflow

1. Simply include `tsflink` evidence collection/linking GitHub action in release workflow (or any workflow that should generate quality documentation).
2. `tsflink` will invoke `trudag publish` for TSF report generation; this publication run utilizes evidence renderer plugins provided by `tsftemplate`.
3. The `tsftemplate` release workflow stores the generated report as part of the release artifacts, and deploys the new mkdocs page via GitHub pages.

### Stage III - Associated tools, related project files

- `.dotstop_extensions/references.py`: this is Python code providing the custom evidence report-renderers used by `tsftemplate` (e.g. the OFT renderer)
- `.github/actions/run-oft/`: Wrapper for running the OpenFastTrace tool, for convenience and improved clarity of the release workflow.
- `.github/workflows/release.yaml`: the `tsftemplate` release workflow, runs OpenFastTrace to get a complete requirements trace, uses `tsffer` to tidy up tsffer artifacts from across all possible pipeline jobs and steps, `tsflink` to invoke TSF graph scoring and report publication.
- `.github/workflows/docs.yaml`: build mkdocs site including TSF report, and publish to GH pages.
- `mkdocs/`: configuration and assets related to the mkdocs project/TSF website.
- `.env.oft`: provide some configuration for the OpenFastTrace reporting tool.
- `mkdocs.yaml`: mkdocs configuration.
