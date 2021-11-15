FROM gitpod/workspace-full:latest

USER root

### kubectl ###
RUN curl -fsSL https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key add - \
    # really 'xenial'
    && add-apt-repository -yu "deb https://apt.kubernetes.io/ kubernetes-xenial main" \
    && install-packages kubectl=1.20.0-00 \
    && kubectl completion bash > /usr/share/bash-completion/completions/kubectl

RUN curl -fsSL -o /usr/bin/kubectx https://raw.githubusercontent.com/ahmetb/kubectx/master/kubectx && chmod +x /usr/bin/kubectx \
    && curl -fsSL -o /usr/bin/kubens  https://raw.githubusercontent.com/ahmetb/kubectx/master/kubens  && chmod +x /usr/bin/kubens

RUN curl -fsSL https://github.com/kubernetes-sigs/kubebuilder/releases/download/v2.3.2/kubebuilder_2.3.2_linux_amd64.tar.gz | tar -xz -C /tmp/ \
    && sudo mkdir -p /usr/local/kubebuilder \
    && sudo mv /tmp/kubebuilder_2.3.2_linux_amd64/* /usr/local/kubebuilder \
    && rm -rf /tmp/*

### Google Cloud ###
# not installed via repository as then 'docker-credential-gcr' is not available
ARG GCS_DIR=/opt/google-cloud-sdk
ENV PATH=$GCS_DIR/bin:$PATH
RUN sudo chown gitpod: /opt \
    && mkdir $GCS_DIR \
    && curl -fsSL https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-sdk-354.0.0-linux-x86_64.tar.gz \
    | tar -xzvC /opt \
    && /opt/google-cloud-sdk/install.sh --quiet --usage-reporting=false --bash-completion=true \
    --additional-components docker-credential-gcr alpha beta \
    # needed for access to our private registries
    && docker-credential-gcr configure-docker

# Set Application Default Credentials (ADC) based on user-provided env var
RUN echo ". /workspace/flagger-playground/scripts/setup-google-adc.sh" >> ~/.bashrc

# Adjust permissions
RUN chown -R gitpod:gitpod ~/.config
RUN chown -R gitpod:gitpod ~/.docker

### Helm3 ###
RUN mkdir -p /tmp/helm/ \
    && curl -fsSL https://get.helm.sh/helm-v3.7.1-linux-amd64.tar.gz | tar -xzvC /tmp/helm/ --strip-components=1 \
    && cp /tmp/helm/helm /usr/local/bin/helm \
    && cp /tmp/helm/helm /usr/local/bin/helm3 \
    && rm -rf /tmp/helm/ \
    && helm completion bash > /usr/share/bash-completion/completions/helm

USER gitpod

# Install 'cargo watch' for better dev experience
RUN bash -c "cargo install cargo-watch cargo-edit"
# Install musl for building a static binary
RUN bash -c "rustup target add x86_64-unknown-linux-musl"

# install istioctl
RUN bash -c "curl -sL https://istio.io/downloadIstioctl | sh -"
ENV PATH=$PATH:/home/gitpod/.istioctl/bin