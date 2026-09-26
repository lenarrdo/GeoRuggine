# Use Case - Account Creation

|                  |                                                                        |
| ---------------- | ---------------------------------------------------------------------- |
| Precondition     | The user _doesn't have an account_.                                    |
| Postcondition    | The user has its personal account and is authenticated.                |
| Nominal Scenario | The user creates an account by                                         |
| Variants         |                                                                        |
| Exceptions       | 2a: Invalid Username, 2b: Username already taken, 3a: Invalid password |

## Nominal Scenario - The account is successfully created

|               |                                                         |
| ------------- | ------------------------------------------------------- |
| Precondition  | The user _doesn't have an account_.                     |
| Postcondition | The user has its personal account and is authenticated. |

**Steps**

| Step | User Action                                  | System Action                                                          |
| ---- | -------------------------------------------- | ---------------------------------------------------------------------- |
| 1    | The user visits the "Register" page          | The System displays the account creation form.                         |
| 2    | The user inputs their username and password. | The System ensures that the username is valid and unique.              |
| 3    |                                              | The System ensures that the password is valid.                         |
| 4    |                                              | The System registers the account in the database and logs the user in. |

## Exception 2a - Invalid username

> [!NOTE]
> A username must contain only letters and numbers.

|               |                                                                       |
| ------------- | --------------------------------------------------------------------- |
| Precondition  | The user _doesn't have an account_.                                   |
| Postcondition | The user still doesn't have an account and an error message is shown. |

**Steps**

| Step | User Action                                  | System Action                                                                       |
| ---- | -------------------------------------------- | ----------------------------------------------------------------------------------- |
| 2a.1 | The user inputs their username and password. | The System displays an error message telling the user that the username is invalid. |

## Exception 2b - Username already taken

|               |                                                                       |
| ------------- | --------------------------------------------------------------------- |
| Precondition  | The user _doesn't have an account_.                                   |
| Postcondition | The user still doesn't have an account and an error message is shown. |

**Steps**

| Step | User Action                                  | System Action                                                                             |
| ---- | -------------------------------------------- | ----------------------------------------------------------------------------------------- |
| 2b.1 | The user inputs their username and password. | The System displays an error message telling the user that the username is already taken. |

## Exception 3a - Invalid password

> [!NOTE]
> A password must be longer than 6 characters and must contain at least an uppercase letter.

|               |                                                                       |
| ------------- | --------------------------------------------------------------------- |
| Precondition  | The user _doesn't have an account_.                                   |
| Postcondition | The user still doesn't have an account and an error message is shown. |

**Steps**

| Step | User Action                                  | System Action                                                                                    |
| ---- | -------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| 3a   | The user inputs their username and password. | The System displays an error message telling what a valid password should look live (see above). |
