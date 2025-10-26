// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../contracts/PolyEndpoint.sol";

contract PolyEndpointTest is Test {
    PolyEndpoint polyEndpoint;
    address owner;
    address admin1;
    address admin2;
    address user;

    function setUp() public {
        owner = address(this); // Test contract is the owner
        admin1 = address(0x1);
        admin2 = address(0x2);
        user = address(0x3);

        polyEndpoint = new PolyEndpoint();
    }

    // Deployment tests
    function testOwnerIsSet() public view {
        assertEq(polyEndpoint.owner(), owner);
    }

    function testInitialEndpointCount() public view {
        assertEq(polyEndpoint.getEndpointCount(), 0);
    }

    // Admin management tests (Owner only)
    function testAddAdmin() public {
        vm.prank(owner);
        polyEndpoint.addAdmin(admin1);
        assertTrue(polyEndpoint.admins(admin1));
    }

    function testRemoveAdmin() public {
        vm.prank(owner);
        polyEndpoint.addAdmin(admin1);
        
        vm.prank(owner);
        polyEndpoint.removeAdmin(admin1);
        assertFalse(polyEndpoint.admins(admin1));
    }

    function testFailAddAdminByNonOwner() public {
        vm.prank(user);
        polyEndpoint.addAdmin(admin1);
    }

    function testFailAddAdminZeroAddress() public {
        vm.prank(owner);
        polyEndpoint.addAdmin(address(0));
    }

    function testFailAddDuplicateAdmin() public {
        vm.prank(owner);
        polyEndpoint.addAdmin(admin1);
        
        vm.prank(owner);
        polyEndpoint.addAdmin(admin1);
    }

    // Endpoint management tests (Admin/Owner only)
    function testAddEndpoint() public {
        polyEndpoint.addEndpoint("https://api.example.com");
        assertEq(polyEndpoint.getEndpointCount(), 1);
        assertTrue(polyEndpoint.hasEndpoint("https://api.example.com"));
    }

    function testAddEndpointByAdmin() public {
        // Add admin
        vm.prank(owner);
        polyEndpoint.addAdmin(admin1);

        // Admin adds endpoint
        vm.prank(admin1);
        polyEndpoint.addEndpoint("https://api.admin.com");
        assertTrue(polyEndpoint.hasEndpoint("https://api.admin.com"));
    }

    function testRemoveEndpoint() public {
        polyEndpoint.addEndpoint("https://api1.example.com");
        polyEndpoint.addEndpoint("https://api2.example.com");

        polyEndpoint.removeEndpoint("https://api1.example.com");
        
        assertEq(polyEndpoint.getEndpointCount(), 1);
        assertFalse(polyEndpoint.hasEndpoint("https://api1.example.com"));
        assertTrue(polyEndpoint.hasEndpoint("https://api2.example.com"));
    }

    function testFailAddEmptyEndpoint() public {
        polyEndpoint.addEndpoint("");
    }

    function testFailAddDuplicateEndpoint() public {
        polyEndpoint.addEndpoint("https://api.example.com");
        polyEndpoint.addEndpoint("https://api.example.com");
    }

    function testFailRemoveNonExistentEndpoint() public {
        polyEndpoint.removeEndpoint("https://api.example.com");
    }

    function testFailAddEndpointByNonAdmin() public {
        vm.prank(user);
        polyEndpoint.addEndpoint("https://api.example.com");
    }

    function testFailRemoveEndpointByNonAdmin() public {
        polyEndpoint.addEndpoint("https://api.example.com");
        
        vm.prank(user);
        polyEndpoint.removeEndpoint("https://api.example.com");
    }

    // View function tests
    function testGetAllEndpoints() public {
        polyEndpoint.addEndpoint("https://api1.com");
        polyEndpoint.addEndpoint("https://api2.com");
        polyEndpoint.addEndpoint("https://api3.com");

        string[] memory endpoints = polyEndpoint.getAllEndpoints();
        assertEq(endpoints.length, 3);
        assertEq(endpoints[0], "https://api1.com");
        assertEq(endpoints[1], "https://api2.com");
        assertEq(endpoints[2], "https://api3.com");
    }

    function testGetEndpointByIndex() public view {
        polyEndpoint.addEndpoint("https://api1.com");
        polyEndpoint.addEndpoint("https://api2.com");
        
        string memory endpoint = polyEndpoint.getEndpoint(1);
        assertEq(endpoint, "https://api2.com");
    }

    function testFailGetEndpointOutOfBounds() public view {
        polyEndpoint.getEndpoint(10);
    }

    function testHasEndpoint() public {
        polyEndpoint.addEndpoint("https://api.example.com");
        
        assertTrue(polyEndpoint.hasEndpoint("https://api.example.com"));
        assertFalse(polyEndpoint.hasEndpoint("https://api2.example.com"));
    }

    function testGetEndpointCount() public {
        assertEq(polyEndpoint.getEndpointCount(), 0);
        
        polyEndpoint.addEndpoint("https://api1.com");
        assertEq(polyEndpoint.getEndpointCount(), 1);
        
        polyEndpoint.addEndpoint("https://api2.com");
        assertEq(polyEndpoint.getEndpointCount(), 2);
    }

    // Ownership management tests
    function testTransferOwnership() public {
        vm.prank(owner);
        polyEndpoint.transferOwnership(admin1);
        
        assertEq(polyEndpoint.owner(), admin1);
    }

    function testFailTransferOwnershipByNonOwner() public {
        vm.prank(user);
        polyEndpoint.transferOwnership(admin1);
    }

    function testFailTransferToZeroAddress() public {
        vm.prank(owner);
        polyEndpoint.transferOwnership(address(0));
    }

    // Complex scenario tests
    function testMultipleAdminsAndEndpoints() public {
        // Add multiple admins
        vm.prank(owner);
        polyEndpoint.addAdmin(admin1);
        
        vm.prank(owner);
        polyEndpoint.addAdmin(admin2);

        // Each admin adds endpoints
        vm.prank(admin1);
        polyEndpoint.addEndpoint("https://admin1-endpoint.com");
        
        vm.prank(admin2);
        polyEndpoint.addEndpoint("https://admin2-endpoint.com");

        vm.prank(owner);
        polyEndpoint.addEndpoint("https://owner-endpoint.com");

        // Verify all endpoints exist
        assertEq(polyEndpoint.getEndpointCount(), 3);
        assertTrue(polyEndpoint.hasEndpoint("https://admin1-endpoint.com"));
        assertTrue(polyEndpoint.hasEndpoint("https://admin2-endpoint.com"));
        assertTrue(polyEndpoint.hasEndpoint("https://owner-endpoint.com"));

        // Admin1 removes their endpoint
        vm.prank(admin1);
        polyEndpoint.removeEndpoint("https://admin1-endpoint.com");

        // Verify
        assertEq(polyEndpoint.getEndpointCount(), 2);
        assertFalse(polyEndpoint.hasEndpoint("https://admin1-endpoint.com"));
    }

    function testFuzzAddEndpoint(string memory endpoint) public {
        // Only test non-empty endpoints
        if (bytes(endpoint).length == 0) return;
        
        polyEndpoint.addEndpoint(endpoint);
        assertTrue(polyEndpoint.hasEndpoint(endpoint));
    }
}
