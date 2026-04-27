A GitHub Action that runs OpenFastTrace CLI's `trace` command on the local workspace and uploads the report as a workflow artifact.

Considers specification, implementation and documentation artifacts as defined by a list of glob patterns.

The action uses the [standard OpenFastTrace Action](https://github.com/itsallcode/openfasttrace-github-action) for running OpenFastTrace (OFT).

# Inputs

| Name            | Description                                                                                           |
| :-------------- | :---------------------------------------------------------------------------------------------------- |
| `file-patterns` | A whitespace separated list of glob patterns which specify the files to include in the OFT trace run. |
| `tags`          | A comma separated list of tags to use for [filtering specification items](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide.md#distributing-the-detailing-work).<br>If not set explicitly, all specification items from files matching the file patterns are considered. |
| `report-format` | Format of the generated report. Must be one of `plain`, `html`, or `aspec`. Defaults to `html`.       |

# Outputs

| Name                  | Description                                                                                                                                                 |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `oft-exit-code`       | 0: OFT has run successfully and all specification items are covered<br> 1: OFT has either failed to run or at least one specification item is not covered.  |
| `tracing-report-name` | The filename of the OpenFastTrace report (e.g. `oft-report.html`).                                                                                          |
| `tracing-report-url`  | The URL pointing to the report that has been created by OFT; uploaded via `actions/upload-artifact`.<br>Also available via `actions/download-artifact` with asset-name `oft-tracing-report-{{report-format}}` |
