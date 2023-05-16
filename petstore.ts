export interface Order {
  id?: number;

  petId?: number;

  quantity?: number;

  shipDate?: string;

  status?: string;

  complete?: boolean;
}

export interface Customer {
  id?: number;

  username?: string;

  address?: Array<Address>;
}

export interface Address {
  street?: string;

  city?: string;

  state?: string;

  zip?: string;
}

export interface Category {
  id?: number;

  name?: string;
}

export interface User {
  id?: number;

  username?: string;

  firstName?: string;

  lastName?: string;

  email?: string;

  password?: string;

  phone?: string;

  userStatus?: number;
}

export interface Tag {
  id?: number;

  name?: string;
}

export interface Pet {
  id?: number;

  name: string;

  category?: Category;

  photoUrls: Array<string>;

  tags?: Array<Tag>;

  status?: string;
}

export interface ApiResponse {
  code?: number;

  type?: string;

  message?: string;
}

export interface paths {
  "/pet": {
    post: {
      RequestBody: Pet;
      Response: Pet;
    };
  };
  "/pet/findByStatus": {
    get: {
      Parameters: {
        Query: {
          /**
@description Status values that need to be considered for filter
          */
          status?: string;
        };
      };
      Response: Array<Pet>;
    };
  };
  "/pet/findByTags": {
    get: {
      Parameters: {
        Query: {
          /**
@description Tags to filter by
          */
          tags?: Array<string>;
        };
      };
      Response: Array<Pet>;
    };
  };
  "/pet/{petId}": {
    get: {
      Parameters: {
        Path: {
          /**
@description ID of pet to return
          */
          petId: number;
        };
      };
      Response: Pet;
    };

    post: {
      Parameters: {
        Query: {
          /**
@description Name of pet that needs to be updated
          */
          name?: string;

          /**
@description Status of pet that needs to be updated
          */
          status?: string;
        };
        Path: {
          /**
@description ID of pet that needs to be updated
          */
          petId: number;
        };
      };
    };

    delete: {
      Parameters: {
        Path: {
          /**
@description Pet id to delete
          */
          petId: number;
        };
        Header: {
          /**
@description Pet id to delete
          */
          petId: number;
        };
      };
    };
  };
  "/pet/{petId}/uploadImage": {
    post: {
      Parameters: {
        Query: {
          /**
@description Additional Metadata
          */
          additionalMetadata?: string;
        };
        Path: {
          /**
@description ID of pet to update
          */
          petId: number;
        };
      };
      Response: ApiResponse;
    };
  };
  "/store/inventory": {
    get: {
      Response: {};
    };
  };
  "/store/order": {
    post: {
      RequestBody: Order;
      Response: Order;
    };
  };
  "/store/order/{orderId}": {
    get: {
      Parameters: {
        Path: {
          /**
@description ID of order that needs to be fetched
          */
          orderId: number;
        };
      };
      Response: Order;
    };

    delete: {
      Parameters: {
        Path: {
          /**
@description ID of the order that needs to be deleted
          */
          orderId: number;
        };
      };
    };
  };
  "/user": {
    post: {
      RequestBody: User;
    };
  };
  "/user/createWithList": {
    post: {
      RequestBody: Array<User>;
      Response: User;
    };
  };
  "/user/login": {
    get: {
      Parameters: {
        Query: {
          /**
@description The user name for login
          */
          username?: string;

          /**
@description The password for login in clear text
          */
          password?: string;
        };
      };
      Response: string;
    };
  };
  "/user/logout": {
    get: {};
  };
  "/user/{username}": {
    get: {
      Parameters: {
        Path: {
          /**
@description The name that needs to be fetched. Use user1 for testing.
          */
          username: string;
        };
      };
      Response: User;
    };

    delete: {
      Parameters: {
        Path: {
          /**
@description The name that needs to be deleted
          */
          username: string;
        };
      };
    };
  };
}
