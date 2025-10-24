### On-chain counting
### Project Name: Counting 
### Value Proposition:

The counting program facilitates the ability for any human, regardless of cognitive abilities, to keep track of a number.
The number can be incremented and decremented, enabling full control of the count for the individual.

### Product-Market Fit:

Current human brain capabilities varies from individual to individual, where even humans with high cognitive brain capabilities occasionally suffers from Short Term Memory Loss (STML).
Therefore, any individual can store and interact with an on-chain count, with the ability to both read and modify it freely.

### Target User Profiles:
* The "human" user: This user is any human, regardless of cognitive abilities. 
They value the ability to read and modify their count, without any restrictions.

### User Story
* As a new user, when I register, all required accounts are created.
* As a new user, when I increment/decrement my account, an error will occur.
* As a registered user, when I increment/decrement, the account value will increment/decrement.
* As a registered user, when I create a new account, an error will occur.
* As a new/registered user, when I create/increment/decrement an account that do not belong to me, an error will occur.

### How To Run
1. Clone and enter the repo
```
git clone https://github.com/CarlTornberg/TurbinePB_Q425_CarlTornberg
cd TurbinePB_Q425_CarlTornberg
```
2. Install dependencies
```
yarn install
```

3. Run/test
```
anchor test
 ``` 
