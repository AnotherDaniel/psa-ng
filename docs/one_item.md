# Following one quality item through the process

To shed some practical light on how the `tsftemplate` flow works from end to end, in this document we will follow one quality-item through the entire process: from formulation of the [Trustable Software Framework (TSF)](https://gitlab.eclipse.org/eclipse/tsf/tsf) statement, creation of a corresponding project requirement, implementation of that requirement and link-up of supporting evidence, to publication and publication of all this tracing information in the project documentation and TSF report.

For this exercise we pick the Eclipse Foundation-derived demand that every EF project shall provide a README with information about the project. Please note that this is a very simple example requirement - actual, technical needs will typically result in much more involved breakdown of requirements, detailed specification and elaborate implementation and testing efforts. This documentation is mainly aiming to illustrate the mechanics around efficiently applying TSF, rather than the intricacies of requirement engineering.

## Trustable Software Framework background for project-README requirement

One of the [core TSF tenets](https://pages.eclipse.dev/eclipse/tsf/tsf/objectives/tsf_objectives.html) (TT-EXPECTATIONS) states that
> ["Documentation is provided, specifying what XYZ is expected to do, and what it must not do, and how this is verified."](../trustable/upstream/UPSTREAM.TSF.TT-EXPECTATIONS.md)

A similar demand is made by the [Eclipse Foundation Project Handbook (section Incubation / Development)](https://www.eclipse.org/projects/handbook/#incubation-development), which has been codified into the [ECLIPSE-PROJECT_README statement](../trustable/upstream/UPSTREAM.ECLIPSE.ECLIPSE-PROJECT_README.md), as part of a Eclipse-Process TSF statement library. Therefore, in our `tsftemplate` TSF graph, the `ECLIPSE-PROJECT_README` node links back to `TT-EXPECTATIONS` as it's parent, and together provide the context for the project-level need for a comprehensive README describing project goals and scope.

To read more about the idea for TSF statement libraries, please refer to the [corresponding section of the main README](three_stages.md#stage-i).

## Deriving our own project-level TSF statement

To manifest our acceptance of the need for a project-level README, and have a place for potential customization and adding of further details, we create a project-level TSF statement `TSFTEMPLATE-PROJECT_README`, using the TSF trudag tool:

```shell
trudag manage create-item TSFTEMPLATE PROJECT_README trustable/tsftemplate
```

This command results in a new markdown snippet being created in our project tree: `./trustable/tsftemplate/TSFTEMPLATE-PROJECT_README.md`
The next step is to elaborate the ambition represented by this new item, by providing a statement of expectation and related front-matter configuration:

```markdown
---
normative: true

publish:
    group: "Eclipse Process"
#EVIDENCE_REF#
score: 
    AnotherDaniel: 0.8
---

Project comes with a comprehensive README file, explaining goal, scope, and providing getting-started documentation.
```

There are three noteworthy aspects to this step:

1. The actual content - this should be a simple and falsifiable statement, resulting in elaborate sub-graphs where more general top-level statements are broken down into actionable and provable parts (refer to the [TSF method documentation](https://pages.eclipse.dev/eclipse/tsf/tsf/tools/usage.html#setting-expectations) for [more details](https://pages.eclipse.dev/eclipse/tsf/tsf/tools/usage.html#recording-reasoning)).
2. The front-matter scoring assessment: this is where subject-matter experts are required to [provide their valuation](https://pages.eclipse.dev/eclipse/tsf/tsf/tools/usage.html#sme-review) of how much this node - if proven to be implemented - contributes to the stated quality goal.
3. The `tsftemplate`/`tsflink`-specific placeholder for evidence-references (`#EVIDENCE_REF#`); this is where `tsflink` will later insert any available pointers to evidence for this statement (see below).

The creation of TSF statements is the first step of [operationally applying TSF](three_stages.md#stage-ii) in a `tsftemplate`-style process.

Other front-matter aspects, like the publish-group configuration, will control formal aspects of TSF report generation. For more information, refer to the [TSF documentation](https://pages.eclipse.dev/eclipse/tsf/tsf/).

## Setting up a matching requirement in the project specification

In this example project, we chose to implement formal requirement tracing, support by the [OpenFastTrace](https://github.com/itsallcode/openfasttrace) open source tool. In practice, this boils down to the following:

- we explicitly formulate what our project is supposed to do in the form of a requirements specification (with each requirement assigned a unique identifier)
- this requirement identifier is subsequently referenced in the code, where we implement the requirement, and in the tests for the implementation
- the OpenFastTrace tool uses these requirement-markers to track whether we've provided an implementation for each requirement, and a corresponding test case

This approach provides a very straightforward way to track implementation status and confidence against a written target statement, with tool-based automation options for reporting and dashboard scenarios.

Our requirements are defined in a little [specification document](./specification.md#project-readme), with the README-requirement assigned the id `req~project-readme~1`. Along with the actual requirement text, we also specify that this requirement needs an implementation ("`Needs: impl`"); if we were specifying behavior in code, we would likely also specify the need for an accompanying test.

For more details on working with OpenFastTrace, refer to the [OFT user guide](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide.md).

## Implementing our README

README implementation is a lot of straightforward work - `tsftemplate` comes with relatively extensive documentation, the majority of which is part of the top-level README and documents that are linked from there. From the process point of view, the one additional item at this stage is that we are going to add some lines that indicate this README to be an implementation of our `req~project-readme~1` requirement. Using OpenFastTrace with markdown, this is what we're adding at the top of the README:

```markdown
<!--
`impl~project-readme~1`
Covers:
- req~project-readme~1
-->
```

As this is not a helpful annotation to regular readers, we're putting this inside a markdown/html comment section to prevent it from being picked up by the usual markdown rendering engines. Also note that the syntax of this declaration varies, depending on the content-type - for more information and examples, refer to the [OFT user guide](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide.md).

This requirement-tracer is not an inherent TSF thing - rather, we chose to use requirements tracing as one part of the toolbox of how we implement the TSF method and want to provide evidence for TSF statements. Other requirements tracking tools, or generally other ways of evidence gathering, are perfectly valid options for supporting a quality method like TSF.

## Linking project-README related evidence into the CI automation

Having set the scene with the above preparations, our project README now can be linked into the `tsftemplate` approach for collecting and processing evidence to support our `TSFTEMPLATE-PROJECT_README` statement. TSF evidence is provided by records in statement front-matter, as described in the [TSF documentation](https://pages.eclipse.dev/eclipse/tsf/tsf/model/methodology.html#providing-evidence).

However, a key goal of `tsftemplate` is to offer a way to automate this evidence linking process in a way that allows not only for static evidence (we could create a hard reference to our README.md), but also artifacts that are dynamically generated e.g. by a release workflow (we can for instance perform requirements tracing on the `tsftemplate` repository during release, and provide the results for `req~project-readme~1` as part of the TSF evidence). Therefore we will not hard-code the README evidence into `TSFTEMPLATE-PROJECT_README.md`, but use an approach that allows us to record evidence relationships at design time, to resolve and score evidence availability at compile/release time.

To support our `TSFTEMPLATE-PROJECT_README` claim, we decide to do three things:

1. Explicitly add the project README to the release artifacts, so that it becomes a formally and prominently recorded part of `tsftemplate` releases
2. Reference the download URL for this release artifact as one evidence item in `TSFTEMPLATE-PROJECT_README`
3. In addition, we will provide the OFT tracking information relating to `req~project-readme~1`, so that the detailed specification and implementation of this item is recorded as part of the evidence supporting `TSFTEMPLATE-PROJECT_README`

These steps are codified in the `tsftemplate` [release workflow](../.github/workflows/release.yaml#64), in the form of GitHub action calls:

{% raw %}
```yaml
# Add README evidence item to release file set, use URL as evidence [Stage II workflow]
- name: Upload README to release
  uses: svenstaro/upload-release-action@v2
  id: upload_README
  with:
    repo_token: ${{ secrets.GITHUB_TOKEN }}
    file: README.md
    tag: ${{ github.ref }}
- name: tsffer README evidence
  uses: AnotherDaniel/tsffer@v0.5.5
  id: tsffer_README
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  with:
    mode: reference
      reference_properties: |
        {
          "reference_type": "download_url",
          "url": "${{ steps.upload_README.outputs.browser_download_url }}",
          "description": "Project README"
        }
    asset_description: "The tsftemplate README file"
    asset_name: "Project README"
    asset_tsf_ids: "TSFTEMPLATE-PROJECT_README,TSFTEMPLATE-PROJECT_SCOPE"

# Add an OFT requirements item [Stage II workflow]
- name: tsffer OFT requirement evidence
  uses: AnotherDaniel/tsffer@v0.5.5
  id: tsffer_requirement
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  with:
    mode: reference
      reference_properties: |
        {
          "reference_type": "openfasttrace",
            "requirement_id": "req~project-readme"
        }
    asset_description: "Tracing info for an OpenFastTrace requirement"
    asset_name: "OpenFastTrace Requirement"
    asset_tsf_ids: "TSFTEMPLATE-PROJECT_README"
```
{% endraw %}

Evidence-recording is done via [`tsffer` ("TSF transfer")](https://github.com/AnotherDaniel/tsffer) - a GitHub action specifically created for this purpose. Invoking `tsffer` like this will simply generate a little json snippet in a `.tsffer` file, which gets added to the workflow artifacts cache. The example above will result in the two files `Project README.tsffer` and `OpenFastTrace Requirement.tsffer` (the latter shown as an example).

```json
{
  "asset-info": {
    "description": "Tracing info for an OpenFastTrace requirement",
    "reference-properties": {
      "reference_type": "openfasttrace",
      "requirement_id": "req~project-readme"
    },
    "name": "OpenFastTrace Requirement",
    "tsf-ids": [
      "TSFTEMPLATE-PROJECT_README"
    ]
  },
  "context-info": {
    "by-workflow": "Release workflow",
    "by-action": "tsffer_requirement",
    "commit-sha": "ace93855e2d591a9e34bc1dee31404881c43e1a2",
    "ref": "refs/tags/v0.1.1",
    "release-url": "https://github.com/AnotherDaniel/tsftemplate/releases/tag/v0.1.1",
    "repository": "AnotherDaniel/tsftemplate"
  }
}
```

Notice the `"reference-properties"` section - this content is later linked into `TSFTEMPLATE-PROJECT_README` front-matter for TSF graph scoring and report generation. The `asset_tsf_ids`/`tsf-ids` key points to the TSF node this piece of evidence is intended to support.

This recording of TSF statement-evidence relationships is the second step of [operationally applying TSF](three_stages.md#stage-ii) in a `tsftemplate`-style process.

Other examples for dynamically created evidence might include test and test code coverage reports, performance data collected during test runs or component operation, a SPDX manifest of the projects dependency tree, etc.

## Automated publication of TSF report, including details about our project-README

The goal of all the steps described so far is to enable [automated quality metrics tracking and reporting](three_stages.md#stage-iii) via CI workflows. The [`tsftemplate` release workflow](../.github/workflows/release.yaml#144) provides an example implementation of such an automation, which comprises the following steps:

1. Set up and prepare any artifacts that are inputs for TSF graph scoring and report publication
   1. We use `tsffer` again to collect and tidy up any `.tsffer` metadata files that have been created by the release workflow so far, additionally uploading them as a release artifact
   2. We run the OpenFastTrace tool to build a requirements tracing report in a machine-readable format (ASPEC XML), because that report is used by the TSF report generator for rendering `req~project-readme~1` evidence, and make that report available as a release artifact
2. Run the [`tsflink`](https://github.com/AnotherDaniel/tsflink) GitHub action, which is performs all the TSF-related processing
   1. Iterate over all collected `.tsffer` snippets, inserting the evidence references into corresponding TSF items
   2. Use `trudag` to calculate TSF scores and publish the TSF report
   3. (`trudag`) includes a plugin interface for rendering custom reference types, provided as Python code in `.dotstop_extensions/references.py`, as per the [TSF documentation](https://pages.eclipse.dev/eclipse/tsf/tsf/tools/references.html); `tsftemplate` includes a set of such custom renderers like the `openfasttrace` one)
3. Finally, build a [Material-for-mkdocs](https://squidfunk.github.io/mkdocs-material/) website hosting our requirements tracing and TSF reports, alongside regular project documentation

This concludes the path a quality item takes through the `tsftemplate` interpretation of how apply the Trustable Software Framework method.
