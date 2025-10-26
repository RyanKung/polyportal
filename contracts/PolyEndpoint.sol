// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title PolyEndpoint
 * @dev Service registration and discovery contract
 * @notice This contract enables service registration and discovery on blockchain
 */
contract PolyEndpoint {
    address public owner;
    
    // Mapping to track if an address is an admin
    mapping(address => bool) public admins;
    
    // Array to store all registered endpoints
    string[] private endpoints;
    
    // Mapping to check if an endpoint already exists
    mapping(string => bool) private endpointExists;
    
    // Events
    event AdminAdded(address indexed admin);
    event AdminRemoved(address indexed admin);
    event EndpointAdded(string indexed endpoint);
    event EndpointRemoved(string indexed endpoint);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    constructor() {
        owner = msg.sender;
    }

    /**
     * @dev Modifier to restrict access to owner only
     */
    modifier onlyOwner() {
        require(msg.sender == owner, "PolyEndpoint: caller is not the owner");
        _;
    }

    /**
     * @dev Modifier to restrict access to admins and owner
     */
    modifier onlyAdminOrOwner() {
        require(admins[msg.sender] || msg.sender == owner, "PolyEndpoint: caller is not an admin or owner");
        _;
    }

    /**
     * @dev Add a new admin (owner only)
     * @param admin The address to be added as admin
     */
    function addAdmin(address admin) public onlyOwner {
        require(admin != address(0), "PolyEndpoint: admin address cannot be zero");
        require(!admins[admin], "PolyEndpoint: address is already an admin");
        require(admin != owner, "PolyEndpoint: owner is already admin");
        
        admins[admin] = true;
        emit AdminAdded(admin);
    }

    /**
     * @dev Remove an admin (owner only)
     * @param admin The address to be removed from admins
     */
    function removeAdmin(address admin) public onlyOwner {
        require(admins[admin], "PolyEndpoint: address is not an admin");
        
        admins[admin] = false;
        emit AdminRemoved(admin);
    }

    /**
     * @dev Add an endpoint (admin or owner only)
     * @param endpoint The endpoint URL to be added
     */
    function addEndpoint(string memory endpoint) public onlyAdminOrOwner {
        require(bytes(endpoint).length > 0, "PolyEndpoint: endpoint cannot be empty");
        require(!endpointExists[endpoint], "PolyEndpoint: endpoint already exists");
        
        endpoints.push(endpoint);
        endpointExists[endpoint] = true;
        emit EndpointAdded(endpoint);
    }

    /**
     * @dev Remove an endpoint (admin or owner only)
     * @param endpoint The endpoint URL to be removed
     */
    function removeEndpoint(string memory endpoint) public onlyAdminOrOwner {
        require(endpointExists[endpoint], "PolyEndpoint: endpoint does not exist");
        
        // Find and remove the endpoint from the array
        for (uint256 i = 0; i < endpoints.length; i++) {
            if (keccak256(bytes(endpoints[i])) == keccak256(bytes(endpoint))) {
                endpoints[i] = endpoints[endpoints.length - 1];
                endpoints.pop();
                break;
            }
        }
        
        endpointExists[endpoint] = false;
        emit EndpointRemoved(endpoint);
    }

    /**
     * @dev Get the total number of endpoints
     * @return The count of registered endpoints
     */
    function getEndpointCount() public view returns (uint256) {
        return endpoints.length;
    }

    /**
     * @dev Get all registered endpoints
     * @return An array of all endpoint URLs
     */
    function getAllEndpoints() public view returns (string[] memory) {
        return endpoints;
    }

    /**
     * @dev Get an endpoint by index
     * @param index The index of the endpoint
     * @return The endpoint URL at the given index
     */
    function getEndpoint(uint256 index) public view returns (string memory) {
        require(index < endpoints.length, "PolyEndpoint: index out of bounds");
        return endpoints[index];
    }

    /**
     * @dev Check if an endpoint exists
     * @param endpoint The endpoint URL to check
     * @return True if the endpoint exists, false otherwise
     */
    function hasEndpoint(string memory endpoint) public view returns (bool) {
        return endpointExists[endpoint];
    }

    /**
     * @dev Transfer ownership of the contract to a new owner
     * @param newOwner The address of the new owner
     */
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "PolyEndpoint: new owner cannot be zero address");
        require(newOwner != owner, "PolyEndpoint: new owner cannot be same as current owner");
        
        address previousOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(previousOwner, newOwner);
    }
}