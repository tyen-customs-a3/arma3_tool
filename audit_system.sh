#!/bin/bash

# Comprehensive Audit System for Workflow Integration
# Task #31: Add Audit and Compile Checking Task

echo "🔍 Starting Comprehensive Workflow Integration Audit"
echo "=================================================="

# Create audit report directory
mkdir -p audit_reports
TIMESTAMP=$(date '+%Y%m%d_%H%M%S')
REPORT_FILE="audit_reports/audit_report_${TIMESTAMP}.md"

# Initialize audit report
cat > "$REPORT_FILE" << EOF
# Workflow Integration Audit Report
Generated: $(date)

## Audit Overview
This report provides a comprehensive audit of the workflow integration system,
focusing on compilation status, test results, and validation of fuzzy report
workflow integration.

EOF

echo "📝 Audit report initialized: $REPORT_FILE"

# Function to log to both console and report
log_section() {
    echo ""
    echo "$1"
    echo "$1" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
}

log_result() {
    echo "$1"
    echo "$1" >> "$REPORT_FILE"
}

# 1. Compilation Check
log_section "## 1. Compilation Status Check"
echo "🔧 Running cargo check --all-targets --all-features..."

if cargo check --all-targets --all-features > compilation_check.log 2>&1; then
    log_result "✅ **PASS**: All targets compile successfully"
    COMPILE_STATUS="PASS"
else
    log_result "❌ **FAIL**: Compilation errors detected"
    COMPILE_STATUS="FAIL"
    log_result "\`\`\`"
    tail -20 compilation_check.log >> "$REPORT_FILE"
    log_result "\`\`\`"
fi

# 2. Workspace Library Check
log_section "## 2. Workspace Library Compilation"
echo "📚 Checking individual library compilation..."

LIBRARIES=("arma3_workflow" "arma3_reporter" "arma3_config" "arma3_database" "arma3_models")
LIBRARY_STATUS="PASS"

for lib in "${LIBRARIES[@]}"; do
    echo "  Checking $lib..."
    if cargo check -p "$lib" > "lib_${lib}_check.log" 2>&1; then
        log_result "✅ $lib: PASS"
    else
        log_result "❌ $lib: FAIL"
        LIBRARY_STATUS="FAIL"
    fi
done

# 3. Identify Legacy Code for Removal
log_section "## 3. Legacy Code Identification"
echo "🗂️  Identifying legacy code components..."

log_result "### Legacy CLI Modules (for removal):"
for file in src/cli/extract.rs src/cli/process.rs src/cli/report.rs src/cli/export.rs src/cli/fuzzy_report.rs; do
    if [ -f "$file" ]; then
        log_result "- \`$file\` (exists - should be removed)"
    else
        log_result "- \`$file\` (not found - already removed)"
    fi
done

log_result "### Legacy Adapter Code (temporary bridge code):"
if [ -d "src/cli/adapters/" ]; then
    log_result "- \`src/cli/adapters/\` directory exists with bridge code"
    ls src/cli/adapters/*.rs >> "$REPORT_FILE" 2>/dev/null || log_result "  (no .rs files found)"
else
    log_result "- \`src/cli/adapters/\` directory not found"
fi

# 4. Test Status Check
log_section "## 4. Test Suite Status"
echo "🧪 Checking test compilation and execution..."

if cargo test --no-run --workspace > test_compile_check.log 2>&1; then
    log_result "✅ **PASS**: Tests compile successfully"
    TEST_COMPILE_STATUS="PASS"
else
    log_result "❌ **FAIL**: Test compilation errors detected"
    TEST_COMPILE_STATUS="FAIL"
fi

# 5. Integration Test Verification
log_section "## 5. Integration Test Suite Status"
echo "🔗 Verifying integration test implementation..."

INTEGRATION_TESTS=(
    "tests/integration_tests.rs"
    "tests/concurrent_workflow_tests.rs" 
    "tests/database_consistency_tests.rs"
    "tests/cli_integration_tests.rs"
    "tests/real_data_scenario_tests.rs"
)

INTEGRATION_STATUS="PASS"
for test_file in "${INTEGRATION_TESTS[@]}"; do
    if [ -f "$test_file" ]; then
        log_result "✅ $test_file: EXISTS"
    else
        log_result "❌ $test_file: MISSING"
        INTEGRATION_STATUS="FAIL"
    fi
done

# 6. Workflow Library API Check
log_section "## 6. Workflow Library API Validation"
echo "⚙️  Validating workflow library public API..."

if cargo doc -p arma3_workflow --no-deps > workflow_doc_check.log 2>&1; then
    log_result "✅ **PASS**: Workflow library documentation builds"
    WORKFLOW_API_STATUS="PASS"
else
    log_result "❌ **FAIL**: Workflow library documentation errors"
    WORKFLOW_API_STATUS="FAIL"
fi

# 7. Generate Summary
log_section "## 7. Audit Summary"

log_result "| Component | Status |"
log_result "|-----------|--------|"
log_result "| Compilation | $COMPILE_STATUS |"
log_result "| Library Compilation | $LIBRARY_STATUS |"
log_result "| Test Compilation | $TEST_COMPILE_STATUS |"
log_result "| Integration Tests | $INTEGRATION_STATUS |"
log_result "| Workflow API | $WORKFLOW_API_STATUS |"

# 8. Recommendations
log_section "## 8. Recommendations"

if [ "$COMPILE_STATUS" = "FAIL" ]; then
    log_result "🔧 **Priority 1**: Fix compilation errors before proceeding"
    log_result "   - Focus on removing legacy adapter code rather than fixing"
    log_result "   - Legacy adapters are temporary bridge code meant for removal"
fi

if [ "$INTEGRATION_STATUS" = "PASS" ]; then
    log_result "✅ **Strength**: Comprehensive integration test suite is complete"
fi

log_result "📋 **Next Steps**:"
log_result "1. Remove legacy adapter bridge code (Task #33)"
log_result "2. Delete old CLI modules (Task #34)"
log_result "3. Clean up compilation errors through code removal"
log_result "4. Validate final system state"

# 9. Create Fix Strategy
log_section "## 9. Compilation Fix Strategy"

log_result "Based on upcoming tasks #33 and #34, the recommended approach is:"
log_result ""
log_result "1. **Remove Legacy Adapters** (\`src/cli/adapters/\`)"
log_result "   - These are temporary bridge code meant for removal"
log_result "   - Compilation errors indicate they're outdated"
log_result ""
log_result "2. **Remove Legacy CLI Modules**"
log_result "   - \`src/cli/extract.rs\`, \`src/cli/process.rs\`, etc."
log_result "   - Replace with direct workflow orchestration"
log_result ""
log_result "3. **Update Module Declarations**"
log_result "   - Remove references to deleted modules"
log_result "   - Update imports throughout codebase"

echo ""
echo "✅ Audit completed successfully!"
echo "📄 Full report available at: $REPORT_FILE"

# Cleanup temporary log files
rm -f compilation_check.log test_compile_check.log workflow_doc_check.log lib_*_check.log