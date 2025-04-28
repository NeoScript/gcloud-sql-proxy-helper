# StartProx

This is just a simple utility that can help start `cloud-sql-proxy` to connect to sql instances
on google cloud platform. It will allow users to configure a list of instances to connect to
and allow users to connect to a default port value.

## steps to complete

first we want to check to see if a config exists
if it does load it

if it does not then ask user if they want to create an empty one
go to $HOME/.config -- screw windows for now
then create folder called startprox
and a file inside called config.yml
set it with defaults for the exec path and a fake instance
print the resulting file path to the user and shutdown program

if we are loaded in with config:
then show list of instances and have user select
if port is set then ask user for port selection, default to their given port

start proxy and show output
