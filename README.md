# corectl

A tool for running applications on CoreOS.

## Prerequisites

corectl requires a cluster of one or more CoreOS nodes running etcd and fleet.

## Usage

Start by creating a systemd unit for a service you want to deploy. The unit should use the standard systemd template format, using a few special Mustache-style tags (variable names enclosed in double braces). For example:

``` ini
[Unit]
Description=My application
Requires=docker.service
After=docker.service

[Service]
TimeoutStartSec=0
Restart=always
ExecStartPre=-/usr/bin/docker kill {{ name }}-{{ tag }}-%i
ExecStartPre=-/usr/bin/docker rm {{ name }}-{{ tag }}-%i
ExecStart=/usr/bin/docker run --name {{ name }}-{{ tag }}-%i {{ user }}/{{ name }}:{{ tag }}
ExecStop=/usr/bin/docker stop {{ name }}-{{ tag }}-%i

[Install]
WantedBy=multi-user.target

[X-Fleet]
Conflicts={{ service }}@*.service
```

Save the file as `myapp@.service` and submit the unit to the cluster by running:

``` bash
corectl service add myorg/myapp myapp@.service
```

This assumes the image is hosted on the Docker Hub. To use a different registry, specify a hostname or IP:

``` bash
corectl service add example.com/myorg/myapp myapp@.service
```

Now deploy the latest version of your app (in this example, tag 21d1f49) to the cluster:

``` bash
corectl deploy myorg/myapp:21d1f49
```

This will submit and start the following systemd service using fleet:

``` ini
[Unit]
Description=My application
Requires=docker.service
After=docker.service

[Service]
TimeoutStartSec=0
Restart=always
ExecStartPre=-/usr/bin/docker kill myapp-21d1f49-1
ExecStartPre=-/usr/bin/docker rm myapp-21d1f49-1
ExecStart=/usr/bin/docker run --name myapp-21d1f49-1 myorg/myapp:21d1f49
ExecStop=/usr/bin/docker stop myapp-21d1f49-1

[Install]
WantedBy=multi-user.target

[X-Fleet]
Conflicts=myapp-21d1f49@*.service
```

If a previous version of the service was running when this deploy took place, any instances of it will be stopped after the new version has started.

Now that your service is running, you can scale it up:

``` bash
corectl service scale myorg/myapp 3
```

This will start 2 additional instances of the service on the cluster for a total of 3. You can scale the service back down (even to zero instances, effectively stopping it):

```
corecl service scale myorg/myapp 1
```

If you don't need a service anymore, you can remove it:

```
corectl service remove myorg/myapp
```

This will stop any instances of the service that are still running and delete the unit file from the cluster.

## Templates

The variables available to unit file templates correspond to the components of a Docker image name:

* `{{ host }}` - The hostname or IP of the Docker registry. Defaults to the Docker Hub.
* `{{ user }}` - The user namespace of the image on the Docker Hub. Has no value when using official (top-level) images.
* `{{ name }}` - The name of the Docker image.
* `{{ tag }}` - The tag of the Docker image.

For example, the Docker image myorg/myapp:21d1f49 would have a user, name, and tag of myorg, myapp, and 21d1f49, respectively.

## confd integration

When corectl starts new services, it registers data in etcd that can be watched by confd to update other services' configuration. The canonical example of this is updating a load balancer to proxy to multiple dynamic backend services.

Usage of confd is beyond the scope of corectl's documentation. All you should need is the layout of the etcd data registered by corectl. Here is the key hierarchy used:

```
.
|-- /corectl
    |-- /services
        |-- /{{ host }}
            |-- /{{ user }}
                |-- {{ name }}
                    |-- /current
                        |-- /instance-{{ count }}
                            |-- /port-{{ port }}-{{ protocol }}-proto = {{ protocol }}
                            |-- /port-{{ port }}-{{ protocol }}-port = {{ host_port }}
                            |-- /port-{{ port }}-{{ protocol }}-addr = {{ ip_address }}
```

The variables in the hierarchy above have the following meanings:

* `{{ host }}` - The hostname or IP of the Docker registry. Defaults to the Docker Hub, represented with an @.
* `{{ user }}` - The user namespace of the image on the Docker Hub. Represented by an @ when using an official (top-level) image.
* `{{ name }}` - The name of the Docker image.
* `{{ count }}` - The instance number of the running systemd unit.
* `{{ port }}` - The internal port of the service within the container.
* `{{ protocol }}` - The protocol the service expects on the internal port.
* `{{ host_port }}` - The port on the host that is mapped to the service's internal port.
* `{{ ip_address }}` - The IP address of the service on the host.

There will be a set of three keys under the "instance-{{ count }}" path for each exposed port for each instance of the service that is running. For example, a simple web service using the [official nginx image](https://registry.hub.docker.com/_/nginx/) that exposes port 80 for HTTP would result in three keys being set, illustrated here with etcdctl commands:

``` bash
etcdctl get /corectl/services/@/@/nginx/current/instance-1/port-80-tcp-proto
# => tcp
etcdctl get /corectl/services/@/@/nginx/current/instance-1/port-80-tcp-port
# => 49155
etcdctl get /corectl/services/@/@/nginx/current/instance-1/port-80-tcp-addr
# => 172.17.0.82
```

confd can be configured to watch for changes to keys under the "current" path and rewrite configuration files when any of the service mappings change.

## License

[MIT](http://opensource.org/licenses/MIT)
