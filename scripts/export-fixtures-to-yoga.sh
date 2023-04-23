#!/bin/bash

fd -x /bin/bash -c "htmlq -f {} -- '#test-root' | sd test-root {.} > ../yoga_test_fixtures/{}"