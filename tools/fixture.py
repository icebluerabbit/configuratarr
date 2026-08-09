"""Shared minimal OpenAPI spec fixture for tool tests."""

SPEC = {
    "paths": {
        "/api/v3/naming": {
            "get": {
                "responses": {"200": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/NamingResource"}
                }}}}
            }
        },
        "/api/v3/naming/{id}": {
            "get": {
                "responses": {"200": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/NamingResource"}
                }}}}
            },
            "put": {
                "requestBody": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/NamingResource"}
                }}},
                "responses": {"200": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/NamingResource"}
                }}}}
            }
        },
        "/api/v3/indexer": {
            "get": {
                "responses": {"200": {"content": {"application/json": {
                    "schema": {"type": "array", "items": {"$ref": "#/components/schemas/IndexerResource"}}
                }}}}
            },
            "post": {
                "requestBody": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/IndexerResource"}
                }}},
                "responses": {"200": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/IndexerResource"}
                }}}}
            }
        },
        "/api/v3/indexer/{id}": {
            "get": {
                "responses": {"200": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/IndexerResource"}
                }}}}
            },
            "put": {
                "requestBody": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/IndexerResource"}
                }}},
                "responses": {"200": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/IndexerResource"}
                }}}}
            },
            "delete": {"responses": {"200": {}}}
        }
    },
    "components": {
        "schemas": {
            "NamingResource": {
                "properties": {
                    "id": {"type": "integer"},
                    "renameMovies": {"type": "boolean", "default": False},
                    "replaceIllegalCharacters": {"type": "boolean", "default": True},
                    "colonReplacementFormat": {"$ref": "#/components/schemas/ColonReplacementFormat"},
                    "standardMovieFormat": {"type": "string", "nullable": True},
                    "movieFolderFormat": {"type": "string", "nullable": True},
                }
            },
            "IndexerResource": {
                "properties": {
                    "id": {"type": "integer"},
                    "name": {"type": "string", "nullable": True},
                    "fields": {"type": "array", "nullable": True, "items": {"$ref": "#/components/schemas/Field"}},
                    "implementation": {"type": "string", "nullable": True},
                    "implementationName": {"type": "string", "nullable": True, "readOnly": True},
                    "configContract": {"type": "string", "nullable": True},
                    "infoLink": {"type": "string", "nullable": True, "readOnly": True},
                    "enableRss": {"type": "boolean"},
                    "supportsRss": {"type": "boolean", "readOnly": True},
                    "priority": {"type": "integer", "default": 25},
                    "tags": {"type": "array", "nullable": True, "items": {"type": "integer"}},
                }
            },
            "Field": {
                "properties": {
                    "name": {"type": "string"},
                    "value": {"type": "string", "nullable": True},
                }
            },
            "ColonReplacementFormat": {
                "enum": ["delete", "dash", "spaceDash"],
                "type": "string"
            }
        }
    }
}

SPEC2 = {
    "paths": {},
    "components": {
        "schemas": {
            "NamingResource": {
                "properties": {
                    "id": {"type": "integer"},
                    "renameEpisodes": {"type": "boolean"},
                    "standardEpisodeFormat": {"type": "string", "nullable": True},
                    "movieFolderFormat": {"type": "string", "nullable": True},
                }
            }
        }
    }
}
