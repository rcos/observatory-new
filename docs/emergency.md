# Emergency Help

**This document is very RCOS-specific. If you are outside of RCOS this is likely
not relevant for you**

First of all check the [deploying documentation](./deploying.md) so that you
understand the current setup of the server, and how things are supposed to be
deployed.

## 1. Contact Steven

Either send me a message on the RCOS Mattermost `@rushsteve1` or send me an
email using whatever email is listed on my GitHub `@rushsteve1`.

I may not be immediately available, and I may not even be able to help, but I'll
help in any way I can.

## 2. Check the Logs

The Observatory logs it's errors to `stdout`. If you were running in Docker
these can easily be viewed with `docker logs <container name>`.
The logs are going to be *very* helpful as they should tell you exactly what is
going wrong, and often will even tell you what line the error was on.

## 3. Restart the Application

This should be obvious but simply restarting my solve the issue, or at least
bring you back up for now.
If running on Docker using the provided `docker-compose.yml` then the container
will restart on crash.
