# Usage default features:
# tilt up
#
# Usage with features:
# tilt up telemetry
config.define_string("features", args=True)
cfg = config.parse()
features = cfg.get('features', "")
print("compiling with features: {}".format(features))

docker_build('kodemaniak/podinfo-rs', '.', dockerfile='Dockerfile')
k8s_yaml(['test/k8s/deployment.yaml', 'test/k8s/namespace.yaml', 'test/k8s/service.yaml'])
k8s_resource('podinfo-rs', port_forwards=20000)
