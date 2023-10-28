# Shrocker agent

The agent program to do affair stuffs the shared server. I'm planning to make this be able to...

- [ ] Create a new user when requested by the community member (← I'm thinking I will release the first v0.1.0 right after this is completed)
- [ ] Check the container or service's usage information
- [ ] Check the resource data, such as used ports list

## Interface

### Discord Bot

Hence Approvers runs in Discord, I'm thinking to create shrocker Agent Discord Bot.

#### Commands

```
!shr register <user_name> <public_key>
```

Creates a new user on shared server, and provide the connection information in the direct message.

#### Road map

- [ ] Slash commands
- [ ] More commands
