# Use Case - Login

|                  |                                                                              |
| ---------------- | ---------------------------------------------------------------------------- |
| Precondition     | The user _has an account_ and _is not authenticated_.                        |
| Postcondition    | The user is authenticated.                                                   |
| Nominal Scenario | The user inserts their credentials and the system logs them in successfully. |
| Variants         |                                                                              |
| Exceptions       | 2a: Missing username or password; 3a: Incorrect username or password.        |

## Nominal Scenario - Successful Login

| Nominal Scenario | Successful Login                                      |
| ---------------- | ----------------------------------------------------- |
| Precondition     | The user _has an account_ and _is not authenticated_. |
| Postcondition    | The user is authenticated.                            |

**Steps**

| Step | User Action                                | System Action                                                                     |
| ---- | ------------------------------------------ | --------------------------------------------------------------------------------- |
| 1    | The user visits the login page.            | The System displays the login form.                                               |
| 2    | The user inputs its account's credentials. | The System compares the inserted credentials with the ones saved in the database. |
| 3    |                                            | The credentials are correct and the System redirects the user to the homepage.    |

## Exception 2a - Missing username or password

| Nominal Scenario | Successful Login                                      |
| ---------------- | ----------------------------------------------------- |
| Precondition     | The user _has an account_ and _is not authenticated_. |
| Postcondition    | The user remains unauthenticated.                     |

**Steps**

| Step | User Action                                                                             | System Action                                                                           |
| ---- | --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| 2a   | The user inputs only their username or password, leaving (at least) one field unfilled. | The System shows an error message telling the use to fill out the form in its entirety. |

## Exception 3a - Incorrect username or password

| Nominal Scenario | Successful Login                                      |
| ---------------- | ----------------------------------------------------- |
| Precondition     | The user _has an account_ and _is not authenticated_. |
| Postcondition    | The user remains unauthenticated.                     |

**Steps**

| Step | User Action | System Action                                                                                |
| ---- | ----------- | -------------------------------------------------------------------------------------------- |
| 3a   |             | The credentials are incorrect. The System displays an error message and clears the password. |
