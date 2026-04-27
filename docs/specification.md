<!-- 
SPDX-FileCopyrightText: 2026 Contributors to the Eclipse Foundation

See the NOTICE file(s) distributed with this work for additional
information regarding copyright ownership.

This program and the accompanying materials are made available under
the terms of the Apache License Version 2.0 which is available at
https://www.apache.org/licenses/LICENSE-2.0
 
SPDX-FileType: DOCUMENTATION
SPDX-License-Identifier: Apache-2.0
-->

# `tsftemplate` Specification

---

The key words "*MUST*", "*MUST NOT*", "*REQUIRED*", "*SHALL*", "*SHALL NOT*", "*SHOULD*", "*SHOULD NOT*",  "*RECOMMENDED*", "*MAY*", and "*OPTIONAL*" in this document are to be interpreted as described in [IETF BCP14 (RFC2119 & RFC8174)](https://www.rfc-editor.org/info/bcp14)

---

This document is supposed to be an illustration of how the OpenFastTrace requirements tool can be integrated with a Trustable Software Framework automation pipeline, to leverage tool-driven requirements tracing and software quality assessment.

## Project requirements

This section defines formal requirements for the implementation of the tsftemplate project.

### Documentation requirements

The following are the foundational requirements for the `tsftemplate` project documentation.

#### Project README

`req~project-readme~1`:
The project *MUST* provide a comprehensive README file, explaining the project goal and composition.

Needs: impl

#### Project scope definition

`req~project-scope~1`:
The project *MUST* provide a clear and concise scope definition.

Needs: impl

#### Getting-started documentation

`req~project-getting_started~1`:
The project *MUST* provide complete and reproducible getting-started documentation.

Needs: impl

Depends:
- req~project-readme~1
