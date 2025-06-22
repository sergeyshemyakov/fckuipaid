#!/bin/bash

# fckuipaid ZK Proof Generation Test Script
# Tests the end-to-end proof generation system

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SERVER_URL="http://localhost:3001"
HEALTH_ENDPOINT="$SERVER_URL/health"
PROOF_ENDPOINT="$SERVER_URL/generate-proof"

# Test data
TEST_USER_ADDRESS="0x170f6F7b0925CF1447BAAF25a5AE61253EF31c1B"
TEST_PAYMENT_RECEIVER="0x6fEDEb0B4942A8b438AFE68ba7c8Af4637c41903"
TEST_MONTH=72025
TEST_SIGNATURE="0x184bdcdfb9db09b6f55c7bcdd3907e2a8555d599b63289fe1fe014864fe605bf01a0f53bcbc539f2070480162f745c847e77e4bcd8821a39ba416945c1b1666c1b"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Function to check if server is running
check_server() {
    print_status "Checking if server is running on $SERVER_URL..."
    
    if curl -s -f "$HEALTH_ENDPOINT" > /dev/null 2>&1; then
        print_success "Server is running"
        return 0
    else
        print_error "Server is not running or health endpoint not available"
        print_status "Please start the server with: cd backend && cargo run"
        return 1
    fi
}

# Function to check environment setup
check_environment() {
    print_status "Checking environment setup..."
    
    # Check if .env file exists
    if [ ! -f ".env" ]; then
        print_warning ".env file not found. Checking env.example..."
        if [ -f "env.example" ]; then
            print_status "Copying env.example to .env"
            cp env.example .env
            print_warning "Please edit .env file with your configuration before running tests"
        else
            print_error "Neither .env nor env.example found"
            return 1
        fi
    fi
    
    # Check for required environment variables
    source .env 2>/dev/null || true
    
    local missing_vars=()
    
    if [ -z "$PROGRAM_URL" ]; then
        missing_vars+=("PROGRAM_URL")
    fi
    
    if [ -z "$PRIVATE_KEY" ] || [ "$PRIVATE_KEY" = "your_ethereum_private_key_here" ]; then
        missing_vars+=("PRIVATE_KEY")
    fi
    
    if [ -z "$ARB_RPC_URL" ]; then
        missing_vars+=("ARB_RPC_URL")
    fi
    
    if [ ${#missing_vars[@]} -gt 0 ]; then
        print_error "Missing or invalid environment variables:"
        for var in "${missing_vars[@]}"; do
            echo "  - $var"
        done
        print_status "Please set these variables in your .env file"
        return 1
    fi
    
    print_success "Environment setup looks good"
    return 0
}

# Function to test proof generation
test_proof_generation() {
    print_status "Testing ZK proof generation..."
    
    local request_data='{
        "user_address": "'$TEST_USER_ADDRESS'",
        "payment_receiver": "'$TEST_PAYMENT_RECEIVER'",
        "month": '$TEST_MONTH',
        "signature": "'$TEST_SIGNATURE'"
    }'
    
    print_status "Sending request to $PROOF_ENDPOINT"
    print_status "Request data: $request_data"
    
    local response=$(curl -v -w "\n%{http_code}" -X POST "$PROOF_ENDPOINT" \
        -H "Content-Type: application/json" \
        -d "$request_data")
    
    local http_code=$(echo "$response" | tail -n1)
    local response_body=$(echo "$response" | sed '$d')
    
    print_status "HTTP Status Code: $http_code"
    print_status "Response: $response_body"
    
    if [ "$http_code" -eq 200 ]; then
        # Parse JSON response to check success field
        local success=$(echo "$response_body" | grep -o '"success":[^,]*' | cut -d':' -f2 | tr -d ' ')
        
        if [ "$success" = "true" ]; then
            print_success "Proof generation successful!"
            
            # Extract key fields from response
            local proof_id=$(echo "$response_body" | grep -o '"proof_id":"[^"]*"' | cut -d'"' -f4)
            local message=$(echo "$response_body" | grep -o '"message":"[^"]*"' | cut -d'"' -f4)
            
            print_success "Proof ID: $proof_id"
            print_success "Message: $message"
            
            # Check if we got journal, seal, and commitment
            if echo "$response_body" | grep -q '"journal":"[^"]*"' && \
               echo "$response_body" | grep -q '"seal":"[^"]*"' && \
               echo "$response_body" | grep -q '"commitment":"[^"]*"'; then
                print_success "Response contains journal, seal, and commitment data"
            else
                print_warning "Response missing some proof data fields"
            fi
            
            return 0
        else
            print_error "Proof generation failed (success=false)"
            return 1
        fi
    else
        print_error "HTTP request failed with status code: $http_code"
        print_error "Response: $response_body"
        return 1
    fi
}

# Function to test invalid requests
test_invalid_requests() {
    print_status "Testing invalid request handling..."
    
    # Test 1: Missing required field
    print_status "Test 1: Missing user_address field"
    local invalid_request1='{
        "payment_receiver": "'$TEST_PAYMENT_RECEIVER'",
        "month": '$TEST_MONTH',
        "signature": "'$TEST_SIGNATURE'"
    }'
    
    local response1=$(curl -s -w "\n%{http_code}" -X POST "$PROOF_ENDPOINT" \
        -H "Content-Type: application/json" \
        -d "$invalid_request1")
    
    local http_code1=$(echo "$response1" | tail -n1)
    
    if [ "$http_code1" -eq 400 ] || [ "$http_code1" -eq 422 ] || [ "$http_code1" -eq 500 ]; then
        print_success "Invalid request properly rejected (HTTP $http_code1)"
    else
        print_warning "Invalid request not properly handled (HTTP $http_code1)"
    fi
    
    # Test 2: Invalid address format
    print_status "Test 2: Invalid address format"
    local invalid_request2='{
        "user_address": "invalid_address",
        "payment_receiver": "'$TEST_PAYMENT_RECEIVER'",
        "month": '$TEST_MONTH',
        "signature": "'$TEST_SIGNATURE'"
    }'
    
    local response2=$(curl -s -w "\n%{http_code}" -X POST "$PROOF_ENDPOINT" \
        -H "Content-Type: application/json" \
        -d "$invalid_request2")
    
    local http_code2=$(echo "$response2" | tail -n1)
    
    if [ "$http_code2" -eq 400 ] || [ "$http_code2" -eq 422 ] || [ "$http_code2" -eq 500 ]; then
        print_success "Invalid address properly rejected (HTTP $http_code2)"
    else
        print_warning "Invalid address not properly handled (HTTP $http_code2)"
    fi
}

# Function to run performance test
test_performance() {
    print_status "Running basic performance test..."
    
    local request_data='{
        "user_address": "'$TEST_USER_ADDRESS'",
        "payment_receiver": "'$TEST_PAYMENT_RECEIVER'",
        "month": '$TEST_MONTH',
        "signature": "'$TEST_SIGNATURE'"
    }'
    
    print_status "Measuring response time..."
    local start_time=$(date +%s.%N)
    
    local response=$(curl -s -w "\n%{http_code}" -X POST "$PROOF_ENDPOINT" \
        -H "Content-Type: application/json" \
        -d "$request_data")
    
    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc -l)
    
    local http_code=$(echo "$response" | tail -n1)
    
    if [ "$http_code" -eq 200 ]; then
        print_success "Performance test completed in ${duration}s"
        
        # Categorize performance
        if (( $(echo "$duration < 10" | bc -l) )); then
            print_success "Excellent performance (< 10s)"
        elif (( $(echo "$duration < 30" | bc -l) )); then
            print_success "Good performance (< 30s)"
        elif (( $(echo "$duration < 60" | bc -l) )); then
            print_warning "Acceptable performance (< 60s)"
        else
            print_warning "Slow performance (> 60s) - this is expected for ZK proof generation"
        fi
    else
        print_error "Performance test failed with HTTP $http_code"
    fi
}

# Main test runner
main() {
    echo "======================================"
    echo "fckuipaid ZK Proof Generation Tests"
    echo "======================================"
    echo
    
    # Check environment first
    if ! check_environment; then
        print_error "Environment check failed. Please fix the issues above."
        exit 1
    fi
    
    # Check if server is running
    if ! check_server; then
        print_error "Server check failed. Please start the server first."
        exit 1
    fi
    
    echo
    print_status "Starting test suite..."
    echo
    
    # Run tests
    local test_results=()
    
    # Test 1: Basic proof generation
    print_status "=== Test 1: Basic Proof Generation ==="
    if test_proof_generation; then
        test_results+=("✅ Basic proof generation")
    else
        test_results+=("❌ Basic proof generation")
    fi
    echo
    
    # Test 2: Invalid requests
    print_status "=== Test 2: Invalid Request Handling ==="
    if test_invalid_requests; then
        test_results+=("✅ Invalid request handling")
    else
        test_results+=("❌ Invalid request handling")
    fi
    echo
    
    # Test 3: Performance test
    print_status "=== Test 3: Performance Test ==="
    if test_performance; then
        test_results+=("✅ Performance test")
    else
        test_results+=("❌ Performance test")
    fi
    echo
    
    # Print summary
    echo "======================================"
    echo "Test Results Summary"
    echo "======================================"
    for result in "${test_results[@]}"; do
        echo "$result"
    done
    echo
    
    # Count passed tests
    local passed=$(echo "${test_results[@]}" | grep -o "✅" | wc -l)
    local total=${#test_results[@]}
    
    if [ "$passed" -eq "$total" ]; then
        print_success "All tests passed! ($passed/$total)"
        exit 0
    else
        print_warning "Some tests failed. ($passed/$total passed)"
        exit 1
    fi
}

# Run tests if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi 